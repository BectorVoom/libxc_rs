//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2287/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2287(t46528: f64, t816: f64, t4159: f64, t9541: f64, t120: f64, t12971: f64, t13173: f64, t13177: f64, t13193: f64, t13198: f64, t13302: f64, t2618: f64, t2623: f64, t2643: f64, t2645: f64, t2681: f64, t41355: f64, t41363: f64, t41365: f64, t41373: f64, t41386: f64, t47215: f64, t817: f64, t819: f64, t820: f64, t829: f64, t831: f64, t9642: f64) -> f64 {
    let t47220 = t46528 * t816;
    let t47230 = t9541 * t4159;
    let t47231 = 35.0_f64 / 72.0_f64 * t47230;
    let t47239 = 5.0_f64 / 128.0_f64 * t2623 * t13193 + 5.0_f64 / 256.0_f64 * t2623 * t13198 - t2618 * t13173 / 1024.0_f64 - t817 * t819 * t820 * t47215 / 3072.0_f64 - t47220 * t831 / 1024.0_f64 - t13177 * t2681 / 1024.0_f64 + 7.0_f64 / 1536.0_f64 * t41355 + 595.0_f64 / 3456.0_f64 * t41363 - 119.0_f64 / 4608.0_f64 * t41365 - 119.0_f64 / 4608.0_f64 * t41373 + 119.0_f64 / 2304.0_f64 * t41386 - t47231 + t9642 * t13302 / 128.0_f64 + t2643 * t2645 * t120 * t12971 * t829 / 256.0_f64;
    t47239
}
