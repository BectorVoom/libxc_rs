//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1279/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1279(t119457: f64, t122886: f64, t122911: f64, t122918: f64, t124246: f64, t124255: f64, t124256: f64, t129157: f64, t129160: f64, t129165: f64, t129169: f64, t129193: f64, t129213: f64, t129216: f64, t130848: f64, t130858: f64, t130862: f64, t130866: f64, t130882: f64, t130893: f64, t32798: f64, t33265: f64, t33270: f64, t33277: f64, t34410: f64, t34761: f64, t4241: f64, t640: f64, t644: f64, t8442: f64, t8621: f64, t8881: f64, t8882: f64) -> f64 {
    let t130895 = 5.0_f64 / 27.0_f64 * t124246 - t124255 + 5.0_f64 / 27.0_f64 * t124256 + 5.0_f64 / 27.0_f64 * t130848 - 5.0_f64 / 72.0_f64 * t129157 * t8882 - 5.0_f64 / 72.0_f64 * t129160 * t8882 - 5.0_f64 / 72.0_f64 * t129165 * t8882 - 5.0_f64 / 72.0_f64 * t129169 * t8882 + 5.0_f64 / 27.0_f64 * t130858 - 10.0_f64 / 9.0_f64 * t130862 + 10.0_f64 / 27.0_f64 * t130866 + 5.0_f64 / 12.0_f64 * t122911 * t34761 + 5.0_f64 / 12.0_f64 * t122918 * t34761 + 5.0_f64 / 12.0_f64 * t32798 * t8621 * t8881 * t4241 + 5.0_f64 / 12.0_f64 * t129193 * t33265 - 5.0_f64 / 36.0_f64 * t34410 * t33277 + 5.0_f64 / 18.0_f64 * t129213 * t33270 - 35.0_f64 / 12.0_f64 * t129216 * t8442 * t130882 * t644 + 5.0_f64 / 6.0_f64 * t122886 * t119457 * t130882 * t640 - 20.0_f64 / 27.0_f64 * t130893;
    t130895
}
