//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1287/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1287<F: Float>(t32422: F, t9860: F, t20160: F, t33761: F, t32439: F, t33783: F, t33940: F, t9515: F) -> (F, F, F, F, F, F) {
    let t115073 = 0.34722222222222222222e-2 * t9860 * t32422;
    let t115075 = t20160 * t33761;
    let t115077 = 0.13402777777777777778e-2 * t32439 * t115075;
    let t115078 = t20160 * t33783;
    let t115080 = 0.40208333333333333334e-2 * t32439 * t115078;
    let t115085 = t9515 * t33940;
    (t115073, t115075, t115077, t115078, t115080, t115085)
}
