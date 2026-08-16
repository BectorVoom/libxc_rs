//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 192/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk192(t492: f64, t498: f64, t193: f64, t336: f64, t425: f64, t453: f64, t455: f64, t265: f64) -> (f64, f64, f64) {
    let t500 = t492 * t498 + 1.0_f64;
    let t501 = f64::ln(t500);
    let t504 = t193 * t336 * t501 - t425 + t453 + t455;
    let t505 = t265 < t504;
    let t506 = piecewise3(t505, t504, t265);
    (t500, t506, t504)
}
