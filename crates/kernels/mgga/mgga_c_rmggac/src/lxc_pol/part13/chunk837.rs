//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 837/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk837<F: Float>(t40735: F, t7788: F, t40135: F, t40739: F, t7782: F, t2392: F, t848: F, t262: F, t40488: F, t7835: F, t39373: F, t39056: F, t7844: F, t39876: F, t39060: F, t7785: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t40858 = t7788 * t40735;
    let t40860 = t7788 * t40135;
    let t40862 = t7782 * t40739;
    let t40864 = t2392 * t848;
    let t40865 = t262 * t40864;
    let t40866 = t7782 * t40865;
    let t40868 = t7835 * t40488;
    let t40870 = t7835 * t39373;
    let t40872 = t7844 * t39056;
    let t40874 = t7844 * t39876;
    let t40877 = t7785 * t39060;
    (t40858, t40860, t40862, t40864, t40865, t40866, t40868, t40870, t40872, t40874, t40877)
}
