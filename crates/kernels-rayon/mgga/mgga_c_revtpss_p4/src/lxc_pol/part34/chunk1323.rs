//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1323/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1323(t22914: f64, t7264: f64, t22865: f64, t25983: f64, t108587: f64, t108590: f64, t108592: f64, t108601: f64, t94477: f64, t94484: f64, t94523: f64, t94526: f64, t98218: f64, t98220: f64, t98224: f64, t98260: f64) -> f64 {
    let t114564 = t7264 * t22914;
    let t114566 = t25983 * t22865;
    let t114570 = -t94477 - 0.18292914397043087774e-2_f64 * t98218 + 0.17149607247227894789e-3_f64 * t108587 - 0.27107389498472794076e-4_f64 * t98220 - 0.12004725073059526352e-1_f64 * t108590 + 0.60023625365297631762e-2_f64 * t108592 - 0.34013387707001991332e-1_f64 * t98224 + t94484 - 0.42874018118069736972e-3_f64 * t114564 + 0.25724410870841842183e-2_f64 * t114566 - 35.0_f64 / 72.0_f64 * t98260 - t94523 + t94526 + 0.42874018118069736972e-4_f64 * t108601;
    t114570
}
