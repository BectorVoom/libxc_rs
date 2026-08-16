//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 912/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk912(t1410: f64, t9681: f64, t2383: f64, t2427: f64, t10051: f64, t1443: f64, t683: f64, t9942: f64, t42050: f64, t91: f64, t2404: f64, t2506: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t96510 = t9681 * t1410;
    let t96694 = t2383 * t2427;
    let t96834 = t1443 * t10051;
    let t97078 = t683 * t9942;
    let t97168 = t91 * t42050;
    let t97181 = t2404 * t2506;
    (t96510, t96694, t96834, t97078, t97168, t97181)
}
