//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1331/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1331(t14076: f64, t64975: f64, t1497: f64, t8096: f64, t19818: f64, t18246: f64, t51780: f64, t3724: f64, t63006: f64, t65437: f64, t65440: f64, t67532: f64, t67533: f64, t68872: f64, t68875: f64, t68878: f64, t68880: f64, t68883: f64, t68885: f64) -> (f64, f64, f64, f64, f64) {
    let t70932 = t64975 * t14076;
    let t70941 = t8096 * t1497;
    let t70942 = t70941 * t19818;
    let t70957 = t18246 * t51780;
    let t70960 = t1497 * t3724;
    let t71158 = -t63006 - t65437 - 44.0_f64 / 9.0_f64 * t65440 - t67532 + t67533 - 4.0_f64 / 3.0_f64 * t68872 - 3.0_f64 / 2.0_f64 * t68875 + t68878 + 2.0_f64 / 3.0_f64 * t68880 + t68883 / 2.0_f64 - t68885 / 4.0_f64;
    (t70932, t70942, t70957, t70960, t71158)
}
