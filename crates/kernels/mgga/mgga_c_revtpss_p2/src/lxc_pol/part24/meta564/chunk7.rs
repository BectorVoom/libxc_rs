//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1709/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1709<F: Float>(t6258: F, t6244: F, t1011: F, t1012: F, t1042: F, t15906: F, t19450: F, t23898: F, t3092: F, t3117: F, t3205: F, t3236: F, t3253: F, t371: F, t372: F, t373: F, t42731: F, t42977: F, t42978: F, t43155: F, t55247: F, t6339: F, t67473: F, t67502: F, t67575: F, t78496: F, t79957: F, t80038: F, t80113: F, t80277: F, t87107: F, t87145: F, t88695: F) -> (F, F, F) {
    let t89312 = t6258 * t6258;
    let t89320 = t6244 * t6244;
    let t89351 = -F::cast_from(0.77173232612525526552e-2_f64) * t15906 * t3117 * t19450 * t80277 + t79957 / F::cast_from(216.0_f64) + F::cast_from(0.12862205435420921092e-2_f64) * t3205 * t371 * t372 * t373 * t89312 + F::cast_from(0.25724410870841842184e-2_f64) * t67502 * t6339 + F::cast_from(0.51448821741683684368e-2_f64) * t43155 * t371 * t372 * t373 * t89320 + F::cast_from(0.34299214494455789578e-2_f64) * t80038 + F::cast_from(0.2540682555144873302e-3_f64) * t55247 - F::cast_from(0.57165357490759649296e-3_f64) * t67473 - F::cast_from(0.77173232612525526552e-2_f64) * t42977 * t1042 * t88695 * t42978 - F::cast_from(0.11433071498151929859e-2_f64) * t80113 - F::cast_from(0.34299214494455789578e-2_f64) * t15906 * t3092 * t78496 * t23898 + t1011 * t1012 * t3253 * t87107 / F::cast_from(72.0_f64) - t1011 * t1012 * t42731 * t87145 / F::cast_from(12.0_f64) - t1011 * t1012 * t3236 * t87107 / F::cast_from(48.0_f64) + F::cast_from(0.3811023832717309953e-3_f64) * t67575;
    (t89312, t89320, t89351)
}
