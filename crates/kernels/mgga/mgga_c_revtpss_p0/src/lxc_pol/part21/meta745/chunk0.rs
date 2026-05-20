//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2619/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2619<F: Float>(t3857: F, t5567: F, t1317: F, t13672: F, t2608: F, t512: F, t5566: F, t1856: F, t9544: F, t46975: F, t39483: F, t39520: F, t39528: F, t39531: F, t46970: F, t48223: F, t48224: F, t48226: F, t48228: F, t48231: F, t48232: F, t48234: F) -> (F, F, F, F, F, F) {
    let t48235 = t3857 * t5567;
    let t48236 = F::new(60.0) * t48235;
    let t48237 = t1317 * t13672;
    let t48238 = F::new(12.0) * t48237;
    let t48240 = t512 * t5566 * t2608;
    let t48241 = F::new(3.0) * t48240;
    let t48243 = t512 * t1856 * t9544;
    let t48244 = F::new(240.0) * t46975;
    let t48245 = t46970 - t48223 + t48224 - t39483 + t48226 + t39520 + t48228 + t48231 - t39528 - t48232 + t39531 - t48234 + t48236 + t48238 + t48241 + t48243 + t48244;
    (t48236, t48238, t48241, t48243, t48244, t48245)
}
