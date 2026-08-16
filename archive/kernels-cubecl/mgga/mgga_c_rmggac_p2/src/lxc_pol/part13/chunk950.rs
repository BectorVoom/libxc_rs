//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 950/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk950<F: Float>(t39056: F, t7844: F, t39876: F, t39060: F, t7785: F, t39880: F, t39064: F, t7788: F, t2347: F, t866: F, t262: F, t2350: F, t876: F) -> (F, F, F, F, F, F, F, F, F) {
    let t40872 = t7844 * t39056;
    let t40874 = t7844 * t39876;
    let t40877 = t7785 * t39060;
    let t40879 = t7785 * t39880;
    let t40881 = t7788 * t39064;
    let t40883 = t2347 * t866;
    let t40884 = t262 * t40883;
    let t40885 = t7788 * t40884;
    let t40887 = t2350 * t876;
    (t40872, t40874, t40877, t40879, t40881, t40883, t40884, t40885, t40887)
}
