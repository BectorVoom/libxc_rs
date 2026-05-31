//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1241/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1241<F: Float>(t14469: F, t50943: F, t13793: F, t53229: F, t3165: F, t898: F, t14456: F, t51666: F, t1114: F, t51916: F, t50935: F, t1112: F, t2306: F, t3074: F, t833: F, t837: F) -> (F, F, F, F, F, F, F) {
    let t53508 = t50943 * t14469;
    let t53509 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t53508;
    let t53515 = t53229 * t13793;
    let t53516 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t53515;
    let t53539 = t898 * t3165;
    let t53545 = t51666 * t14456;
    let t53546 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t53545;
    let t53566 = t1114 * t51916;
    let t53571 = t1114 * t50935;
    let t53577 = t3074 * t2306 * t1112 * t837 * t833;
    (t53509, t53516, t53539, t53546, t53566, t53571, t53577)
}
