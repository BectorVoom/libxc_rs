//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1146/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1146<F: Float>(t106: F, t11140: F, t24484: F, t24553: F, t24629: F, t25017: F, t25070: F, t25133: F, t25192: F, t25249: F, t25256: F, t25260: F, t25267: F, t25278: F, t25279: F, t25287: F, t25334: F, t25385: F, t25439: F, t25498: F, t25546: F, t25604: F, t25646: F, t25689: F, t25729: F, t25774: F, t25817: F, t25870: F, t25917: F, t25964: F, t26004: F, t26049: F, t2688: F, t2694: F, t2695: F, t2818: F, t335: F, t3860: F, t7935: F, t7947: F, t7949: F, t7953: F, t7954: F, t8263: F, t908: F, t956: F) -> (F,) {
    let t26057 = 0.27818116767324025134e1 * t106 * (t24484 + t24553 + t24629 + t25017 + t25070 + t25133 + t25192 + t25249) * t335 - 0.11127246706929610054e2 * t106 * t25256 * t956 + 0.33381740120788830161e2 * t106 * t25260 * t2695 - 0.1669087006039441508e2 * t106 * t7935 * t2818 - 0.66763480241577660323e2 * t106 * t25267 * t7949 + 0.66763480241577660323e2 * t11140 * t7954 - 0.11127246706929610054e2 * t106 * t2688 * t8263 + 0.6676348024157766032e2 * t106 * t25278 * t25279 - 0.10014522036236649048e3 * t3860 * t7947 * t2695 * t2818 + 0.16690870060394415081e2 * t106 * t2694 * t25287 + 0.22254493413859220108e2 * t3860 * t7953 * t8263 - 0.27818116767324025134e1 * t106 * t908 * (t25334 + t25385 + t25439 + t25498 + t25546 + t25604 + t25646 + t25689 + t25729 + t25774 + t25817 + t25870 + t25917 + t25964 + t26004 + t26049);
    (t26057,)
}
