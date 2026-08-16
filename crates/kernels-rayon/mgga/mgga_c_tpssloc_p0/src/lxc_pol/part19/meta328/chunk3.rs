//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1172/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1172(t68: f64, t6924: f64, t12012: f64, t12147: f64, t12157: f64, t12160: f64, t12161: f64, t12164: f64, t1345: f64, t1347: f64, t1348: f64, t16186: f64, t1995: f64, t225: f64, t3719: f64, t3734: f64, t3839: f64, t3843: f64, t3844: f64, t3847: f64, t39622: f64, t39892: f64, t40026: f64, t40210: f64, t40211: f64, t40213: f64, t40214: f64, t40217: f64, t40218: f64, t40220: f64, t40235: f64, t5278: f64, t546: f64, t548: f64) -> f64 {
    let t40253 = t68 * t6924;
    let t40270 = -(t40210 + t40211 + t40213 + t40214 + t40217 + t40218 + t40220 + t40235) * t225 * t548 + 12.0_f64 * t12147 * t1348 - 72.0_f64 * t3839 * t3844 + 18.0_f64 * t3839 * t3847 + 240.0_f64 * t1345 * t12157 - 144.0_f64 * t16186 * t12161 + 12.0_f64 * t1345 * t12164 - 360.0_f64 * t546 * t40253 * t40026 + 360.0_f64 * t5278 * t1995 * t3734 * t3719 - 36.0_f64 * t546 * t3843 * t39622 - 48.0_f64 * t5278 * t12160 * t12012 + 3.0_f64 * t546 * t1347 * t39892;
    t40270
}
