//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 591/1468 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk591(t1382: f64, t3366: f64, t2877: f64, t895: f64, t2898: f64, t901: f64, t1645: f64, t888: f64) -> (f64, f64, f64, f64) {
    let t3368 = 2.0_f64 * t1382 * t3366;
    let t3370 = 0.35750489951850426669e0_f64 * t895 * t2877;
    let t3375 = t2898 * t901;
    let t3376 = 0.14896037479937677779e-1_f64 * t3375;
    let t3377 = t1645 * t888;
    (t3368, t3370, t3376, t3377)
}
