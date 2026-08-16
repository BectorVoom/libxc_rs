//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2210/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2210(t22267: f64, t25997: f64, t22255: f64, t7264: f64, t22259: f64, t22276: f64, t7271: f64, t22281: f64, t26024: f64, t6876: f64, t22289: f64, t102498: f64, t98169: f64, t98174: f64, t98181: f64, t98186: f64, t98188: f64) -> f64 {
    let t108566 = t25997 * t22267;
    let t108568 = t7264 * t22255;
    let t108570 = t25997 * t22259;
    let t108572 = t7271 * t22276;
    let t108574 = t7271 * t22281;
    let t108576 = t26024 * t6876;
    let t108578 = t7271 * t22289;
    let t108580 = -t102498 - t98169 + 0.54208002996571016775e-3_f64 * t98174 - t98181 - 0.25410001404642664113e-4_f64 * t108566 - 0.42874018118069736972e-3_f64 * t108568 - 0.25410001404642664113e-4_f64 * t108570 - 0.51448821741683684367e-1_f64 * t108572 + 0.17149607247227894789e-1_f64 * t108574 + 0.20007875121765877254e-2_f64 * t108576 + 0.85748036236139473945e-2_f64 * t108578 + t98186 - t98188;
    t108580
}
