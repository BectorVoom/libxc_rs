//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3275/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3275(t14662: f64, t2723: f64, t14671: f64, t14686: f64, t14931: f64, t18632: f64, t14494: f64, t14791: f64, t1559: f64, t18426: f64, t18637: f64, t2477: f64, t2745: f64, t2754: f64, t4362: f64, t4364: f64, t4365: f64, t51095: f64, t51098: f64, t51100: f64, t51102: f64, t51104: f64, t51106: f64, t61234: f64, t62080: f64, t828: f64, t851: f64) -> (f64, f64) {
    let t62209 = t2723 * t14662;
    let t62216 = t14931 * t14686 * t14671 * t18632;
    let t62231 = 0.85748036236139473944e-2_f64 * t851 * t2477 * t828 * t61234 + 0.34299214494455789578e-2_f64 * t2745 * t14791 * t14494 * t18637 - 35.0_f64 / 54.0_f64 * t51095 + 0.85748036236139473944e-3_f64 * t4362 * t4364 * t4365 * t62209 + 0.10164000561857065645e-3_f64 * t62216 - 0.21437009059034868486e-3_f64 * t2745 * t4364 * t18426 * t2754 + 0.34299214494455789578e-2_f64 * t2745 * t14791 * t1559 * t62080 - 0.22675591804667994222e-1_f64 * t51098 - 0.25692334753583138158e-2_f64 * t51100 + 0.1219527626469539185e-2_f64 * t51102 + 0.7558530601555998074e-1_f64 * t51104 + 7.0_f64 / 6.0_f64 * t51106;
    (t62209, t62231)
}
