//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2342/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2342<F: Float>(t13278: F, t5614: F, t20963: F, t9667: F, t46881: F, t5587: F, t13222: F, t13251: F, t13350: F, t16888: F, t20947: F, t20993: F, t210: F, t2571: F, t2643: F, t2645: F, t2647: F, t4240: F, t46952: F, t46954: F, t5591: F, t58642: F, t58688: F, t58759: F, t58761: F, t58763: F, t67620: F, t776: F, t829: F) -> F {
    let t67976 = t13278 * t5614;
    let t67978 = t9667 * t20963;
    let t67980 = t46881 * t5587;
    let t67988 = -t58642 * t4240 / F::cast_from(1024.0_f64) - t46952 - t46954 + t2643 * t2645 * t67620 * t2647 / F::cast_from(768.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t58759 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t58761 - F::cast_from(35.0_f64) / F::cast_from(192.0_f64) * t58763 + t2643 * t13222 * t58688 * t5591 / F::cast_from(256.0_f64) + t2571 * t210 * t20993 * t776 / F::cast_from(16.0_f64) + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t67976 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t67978 - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t67980 - F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t13251 * t16888 - F::cast_from(5.0_f64) / F::cast_from(256.0_f64) * t2643 * t13350 * t20947 * t829;
    t67988
}
