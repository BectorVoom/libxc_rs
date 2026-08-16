//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2466/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2466(t1040: f64, t21482: f64, t10876: f64, t21396: f64, t248: f64, t3101: f64, t1041: f64, t21138: f64, t3051: f64, t10403: f64, t10408: f64, t1046: f64, t14211: f64, t17607: f64, t18014: f64, t3071: f64, t42388: f64, t43361: f64, t4338: f64, t4343: f64, t4636: f64, t49743: f64, t5873: f64, t5880: f64, t61675: f64, t62079: f64, t70106: f64) -> f64 {
    let t70153 = t21482 * t1040;
    let t70162 = t10876 * t248 * t3101 * t21396;
    let t70166 = t1041 * t248 * t3051 * t21138;
    let t70189 = t70153 * t1046 / 4608.0_f64 + t49743 * t5880 / 192.0_f64 + t17607 * t4636 / 1536.0_f64 - t70162 / 768.0_f64 + t70166 / 1152.0_f64 + t10403 * t3071 * t14211 * t70106 / 384.0_f64 - t10403 * t3071 * t5873 * t4343 / 384.0_f64 + t42388 * t3071 * t62079 * t18014 / 256.0_f64 + 5.0_f64 / 2304.0_f64 * t10403 * t10408 * t5873 * t4338 - t43361 * t3071 * t5873 * t18014 / 256.0_f64 - t61675 / 144.0_f64;
    t70189
}
