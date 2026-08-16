//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1172/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1172(t1882: f64, t35047: f64, t2179: f64, t3565: f64, t7407: f64, t35084: f64, t8392: f64, t35162: f64, t35185: f64, t35033: f64, t9276: f64, t106300: f64, t107627: f64, t139675: f64, t140364: f64, t140370: f64, t140376: f64, t144: f64, t1901: f64, t2142: f64, t27216: f64, t27221: f64, t27263: f64, t33192: f64, t3408: f64, t3425: f64, t3455: f64, t35110: f64, t3590: f64, t446: f64, t49622: f64, t574: f64, t5842: f64, t5935: f64, t605: f64, t63755: f64, t6725: f64, t7339: f64, t7357: f64, t7414: f64) -> (f64, f64, f64) {
    let t149086 = t1882 * t35047;
    let t149093 = t2179 * t7407 * t3565;
    let t149101 = t8392 * t35084;
    let t149110 = t1882 * t35162;
    let t149112 = t1882 * t35185;
    let t149120 = t9276 * t35033;
    let t149129 = -t446 * t574 * t7414 * t3408 / 3.0_f64 + t446 * t574 * t2142 * t35110 / 3.0_f64 - 4.0_f64 / 9.0_f64 * t1901 * t106300 * t27216 + 4.0_f64 / 27.0_f64 * t1901 * t107627 * t27221 + 8.0_f64 / 3.0_f64 * t1901 * t63755 * t7357 * t3455 + t149086 / 27.0_f64 + 2.0_f64 / 3.0_f64 * t446 * t574 * t5935 * t27263 + 2.0_f64 / 3.0_f64 * t446 * t144 * t149093 - 2.0_f64 / 3.0_f64 * t446 * t574 * t6725 * t5842 + 2.0_f64 / 27.0_f64 * t149101 + t1901 * t139675 * t3425 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t49622 * t33192 - t140364 / 9.0_f64 - 2.0_f64 / 9.0_f64 * t149110 + 2.0_f64 / 9.0_f64 * t149112 - 2.0_f64 / 9.0_f64 * t140370 + t446 * t574 * t605 * t7339 * t3565 / 3.0_f64 + 4.0_f64 / 3.0_f64 * t446 * t144 * t149120 + t140376 / 9.0_f64 - t446 * t574 * t3590 * t7339 / 3.0_f64;
    (t149093, t149120, t149129)
}
