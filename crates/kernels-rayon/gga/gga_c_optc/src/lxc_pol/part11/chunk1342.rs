//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1342/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1342(t5059: f64, t24357: f64, t277: f64, t33574: f64, t33596: f64, t52260: f64, t52264: f64, t57251: f64, t57253: f64, t57257: f64, t57260: f64, t57520: f64, t57523: f64, t57525: f64, t95: f64) -> f64 {
    let t58229 = t5059 * t5059;
    let t58237 = 20.0_f64 / 81.0_f64 * t33574 - t57251 + t57253 + t57257 - 0.15506928860942058298e-1_f64 * t95 * t277 * t58229 * t24357 + t57260 + 20.0_f64 / 27.0_f64 * t33596 + t57520 - t57523 - t57525 + 56.0_f64 / 81.0_f64 * t52260 + 8.0_f64 / 9.0_f64 * t52264;
    t58237
}
