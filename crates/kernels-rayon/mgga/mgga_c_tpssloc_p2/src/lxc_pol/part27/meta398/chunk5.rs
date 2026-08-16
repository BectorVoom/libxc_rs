//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1647/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1647(t15540: f64, t4582: f64, t12648: f64, t4987: f64, t13969: f64, t4983: f64, t3515: f64, t486: f64, t5011: f64, t4978: f64, t11709: f64, t11738: f64, t11814: f64, t11825: f64, t1213: f64, t1227: f64, t15524: f64, t15527: f64, t15531: f64, t15535: f64, t1737: f64, t1748: f64, t3490: f64, t3506: f64, t3531: f64, t3536: f64, t4980: f64, t4989: f64, t5014: f64, t5024: f64) -> (f64, f64) {
    let t15541 = t4582 * t15540;
    let t15544 = t4987 * t12648;
    let t15545 = t4582 * t15544;
    let t15548 = t13969 * t4983;
    let t15550 = t3515 * t15548 / 2304.0_f64;
    let t15553 = t486 * t5011;
    let t15554 = t15553 * t4978;
    let t15555 = t4582 * t15554;
    let t15558 = t5024 * t3531 / 432.0_f64 - t11825 * t1748 / 4608.0_f64 + t11814 * t1737 / 3072.0_f64 + t3536 * t5014 / 1536.0_f64 + t15524 + t1213 * t15527 / 3072.0_f64 - t3515 * t15531 / 3072.0_f64 + t11738 * t15535 / 3072.0_f64 + 5.0_f64 / 6912.0_f64 * t3490 * t4989 + 5.0_f64 / 6912.0_f64 * t1227 * t15541 + 5.0_f64 / 13824.0_f64 * t1227 * t15545 - t15550 + t11709 * t4980 / 768.0_f64 + t3506 * t15555 / 768.0_f64;
    (t15553, t15558)
}
