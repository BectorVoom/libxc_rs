//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1024/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1024(t11: f64, t1109: f64, t171: f64, t33446: f64, t173: f64, t35453: f64, t35454: f64, t35455: f64, t150522: f64, t3766: f64, t141058: f64, t33424: f64, t35361: f64) -> (f64, f64, f64, f64, f64) {
    let t150546 = t11 * t1109 * t171;
    let t150547 = t150546 * t33446;
    let t150552 = t35453 * t35454 * t173 * t35455;
    let t150554 = t3766 * t150522;
    let t150558 = t33424 * t141058 * t35361;
    (t150546, t150547, t150552, t150554, t150558)
}
