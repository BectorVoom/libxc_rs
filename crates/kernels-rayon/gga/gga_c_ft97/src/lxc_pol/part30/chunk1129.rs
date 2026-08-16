//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1129/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1129(t35504: f64, t52: f64, t820: f64, t280: f64, t6789: f64, t35916: f64, t35462: f64, t35908: f64, t811: f64, t5009: f64, t816: f64, t150658: f64, t7006: f64) -> (f64, f64, f64, f64, f64) {
    let t153272 = t52 * t35504 * t820;
    let t153275 = t280 * t6789;
    let t153276 = t153275 * t35916;
    let t153280 = t35908 * t35462 * t811;
    let t153283 = t816 * t5009;
    let t153285 = t153283 * t35462 * t820;
    let t153290 = t7006 * t150658;
    (t153272, t153276, t153280, t153285, t153290)
}
