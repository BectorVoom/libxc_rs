//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 872/1029 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk872(t3177: f64, t35091: f64, t9272: f64, t204: f64, t41726: f64, t587: f64, t2487: f64, t6711: f64, t10532: f64, t10533: f64, t41749: f64, t41810: f64, t6716: f64, t6717: f64) -> (f64, f64, f64, f64, f64) {
    let t42226 = t9272 * t35091 * t3177;
    let t42227 = 0.11502877786176224903e1_f64 * t42226;
    let t42230 = 0.18404604457881959845e2_f64 * t587 * t204 * t41726;
    let t42233 = 0.14953741122029092374e3_f64 * t2487 * t6711 * t41726;
    let t42236 = 0.55213813373645879534e2_f64 * t10532 * t10533 * t41749;
    let t42239 = 0.69017266717057349418e1_f64 * t6716 * t6717 * t41810;
    (t42227, t42230, t42233, t42236, t42239)
}
