//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1709/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1709(t6258: f64, t6244: f64, t1011: f64, t1012: f64, t1042: f64, t15906: f64, t19450: f64, t23898: f64, t3092: f64, t3117: f64, t3205: f64, t3236: f64, t3253: f64, t371: f64, t372: f64, t373: f64, t42731: f64, t42977: f64, t42978: f64, t43155: f64, t55247: f64, t6339: f64, t67473: f64, t67502: f64, t67575: f64, t78496: f64, t79957: f64, t80038: f64, t80113: f64, t80277: f64, t87107: f64, t87145: f64, t88695: f64) -> (f64, f64, f64) {
    let t89312 = t6258 * t6258;
    let t89320 = t6244 * t6244;
    let t89351 = -0.77173232612525526552e-2_f64 * t15906 * t3117 * t19450 * t80277 + t79957 / 216.0_f64 + 0.12862205435420921092e-2_f64 * t3205 * t371 * t372 * t373 * t89312 + 0.25724410870841842184e-2_f64 * t67502 * t6339 + 0.51448821741683684368e-2_f64 * t43155 * t371 * t372 * t373 * t89320 + 0.34299214494455789578e-2_f64 * t80038 + 0.2540682555144873302e-3_f64 * t55247 - 0.57165357490759649296e-3_f64 * t67473 - 0.77173232612525526552e-2_f64 * t42977 * t1042 * t88695 * t42978 - 0.11433071498151929859e-2_f64 * t80113 - 0.34299214494455789578e-2_f64 * t15906 * t3092 * t78496 * t23898 + t1011 * t1012 * t3253 * t87107 / 72.0_f64 - t1011 * t1012 * t42731 * t87145 / 12.0_f64 - t1011 * t1012 * t3236 * t87107 / 48.0_f64 + 0.3811023832717309953e-3_f64 * t67575;
    (t89312, t89320, t89351)
}
