//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 632/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk632<F: Float>(t739: F, t9858: F, t2295: F, t6355: F, t1704: F, t27: F, t649: F, t7282: F, t570: F, t8800: F, t1356: F, t2301: F, t2868: F, t1734: F, t36: F) -> (F, F, F, F, F, F, F, F) {
    let t9859 = t739 * t9858;
    let t9860 = 0.11974241701863808564e0 * t9859;
    let t9861 = t6355 * t2295;
    let t9862 = 0.5987120850931904282e-1 * t9861;
    let t9864 = t27 * t649 * t1704;
    let t9865 = t7282 * t9864;
    let t9866 = 0.20455996240684006296e-1 * t9865;
    let t9867 = t8800 * t570;
    let t9868 = t1356 * t9867;
    let t9869 = 0.79828278012425390428e-1 * t9868;
    let t9870 = t2868 * t2301;
    let t9871 = 0.2993560425465952141e-1 * t9870;
    let t9872 = t36 * t1734;
    (t9860, t9862, t9864, t9866, t9867, t9869, t9871, t9872)
}
