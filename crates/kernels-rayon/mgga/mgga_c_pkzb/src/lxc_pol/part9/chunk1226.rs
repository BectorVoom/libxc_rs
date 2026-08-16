//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1226/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1226(t1972: f64, t730: f64, t7527: f64, t1987: f64, t7571: f64, t1957: f64, t7535: f64, t1116: f64, t17312: f64, t21186: f64, t21196: f64, t21217: f64, t21220: f64, t21223: f64, t21225: f64, t21233: f64) -> (f64, f64, f64, f64, f64) {
    let t21313 = 0.35089341735807877242e1_f64 * t730 * t7527 * t1972;
    let t21315 = 0.10526802520742363173e2_f64 * t1987 * t7571;
    let t21318 = 0.10526802520742363173e2_f64 * t730 * t7535 * t1957;
    let t21320 = 0.5848223622634646207e0_f64 * t17312 * t1116;
    let t21321 = t21313 - t21315 - t21318 + t21186 - t21196 + t21217 + t21220 + t21223 + t21225 - t21320 + t21233;
    (t21313, t21315, t21318, t21320, t21321)
}
