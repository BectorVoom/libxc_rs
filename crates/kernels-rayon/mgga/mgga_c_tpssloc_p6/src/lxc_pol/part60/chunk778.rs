//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 778/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk778(t28190: f64, t28236: f64, t533: f64, t1390: f64, t1983: f64, t25: f64, t5527: f64, t1915: f64, t1484: f64, t1530: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t28237 = t28190 + t28236;
    let t28238 = t533 * t28237;
    let t28239 = t28238 * t1390;
    let t28240 = t1983 * t28239;
    let t28241 = t25 * t5527;
    let t28242 = t1915 * t28241;
    let t28248 = t1484 * t1530;
    (t28237, t28239, t28240, t28241, t28242, t28248)
}
