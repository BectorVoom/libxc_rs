//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1261/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1261(t106: f64, t11140: f64, t24484: f64, t24553: f64, t24629: f64, t25017: f64, t25070: f64, t25133: f64, t25192: f64, t25249: f64, t25256: f64, t25260: f64, t25267: f64, t25278: f64, t25279: f64, t25287: f64, t25334: f64, t25385: f64, t25439: f64, t25498: f64, t25546: f64, t25604: f64, t25646: f64, t25689: f64, t25729: f64, t25774: f64, t25817: f64, t25870: f64, t25917: f64, t25964: f64, t26004: f64, t26049: f64, t2688: f64, t2694: f64, t2695: f64, t2818: f64, t335: f64, t3860: f64, t7935: f64, t7947: f64, t7949: f64, t7953: f64, t7954: f64, t8263: f64, t908: f64, t956: f64) -> f64 {
    let t26057 = 0.27818116767324025134e1_f64 * t106 * (t24484 + t24553 + t24629 + t25017 + t25070 + t25133 + t25192 + t25249) * t335 - 0.11127246706929610054e2_f64 * t106 * t25256 * t956 + 0.33381740120788830161e2_f64 * t106 * t25260 * t2695 - 0.1669087006039441508e2_f64 * t106 * t7935 * t2818 - 0.66763480241577660323e2_f64 * t106 * t25267 * t7949 + 0.66763480241577660323e2_f64 * t11140 * t7954 - 0.11127246706929610054e2_f64 * t106 * t2688 * t8263 + 0.6676348024157766032e2_f64 * t106 * t25278 * t25279 - 0.10014522036236649048e3_f64 * t3860 * t7947 * t2695 * t2818 + 0.16690870060394415081e2_f64 * t106 * t2694 * t25287 + 0.22254493413859220108e2_f64 * t3860 * t7953 * t8263 - 0.27818116767324025134e1_f64 * t106 * t908 * (t25334 + t25385 + t25439 + t25498 + t25546 + t25604 + t25646 + t25689 + t25729 + t25774 + t25817 + t25870 + t25917 + t25964 + t26004 + t26049);
    t26057
}
