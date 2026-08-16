//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1057/1242 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1057(t19087: f64, t475: f64, t481: f64, t5623: f64, t5651: f64, t1354: f64, t159: f64, t285: f64, t39: f64, t545: f64, t5668: f64, t143: f64, t1593: f64, t169: f64, t18116: f64, t18122: f64, t18126: f64, t18129: f64, t18131: f64, t18133: f64, t18137: f64, t18140: f64, t18144: f64, t18366: f64, t18987: f64, t18991: f64, t19051: f64, t19083: f64, t2035: f64, t279: f64, t296: f64, t299: f64, t301: f64, t522: f64, t523: f64, t526: f64, t5601: f64, t5625: f64, t5661: f64, t5684: f64) -> f64 {
    let t19088 = t475 * t19087;
    let t19090 = t5651 * t5623 * t481;
    let t19098 = t39 * t1354 * t159 * t285;
    let t19101 = t5668 * t545 * t285;
    let t19103 = -0.21618361918556568284e0_f64 * t18116 - 6.0_f64 * t1593 * t5661 + t18122 + t18126 + 6.0_f64 * t1593 * t5625 - 0.31931290694012290916e0_f64 * t18129 - 0.63862581388024581833e0_f64 * t18131 + 18.0_f64 * t2035 * t18133 + 12.0_f64 * t2035 * t18137 + 36.0_f64 * t5601 * t18140 - t523 * t18144 + (t18366 + t18991) * t279 + t19051 * t296 + 0.20267214298646782767e-1_f64 * t169 * t299 * t18987 * t301 + 3.0_f64 * t475 * t143 * t19083 + 24.0_f64 * t19088 * t19090 + 24.0_f64 * t5684 * t522 * t526 + 0.81358876250083374227e-2_f64 * t19098 + 0.16271775250016674846e-1_f64 * t19101;
    t19103
}
