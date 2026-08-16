//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 876/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk876(t1162: f64, t338: f64, t3907: f64, t1115: f64, t12111: f64, t12195: f64, t12199: f64, t12223: f64, t12246: f64, t12253: f64, t13641: f64, t13645: f64, t13650: f64, t13656: f64, t13662: f64, t13680: f64, t13684: f64, t13688: f64, t2401: f64, t2408: f64, t2503: f64, t335: f64, t3921: f64, t833: f64, t844: f64, t8659: f64, t9820: f64, t9899: f64) -> (f64, f64) {
    let t13695 = t338 * t3907 * t1162;
    let t13698 = -t844 * t13641 / 48.0_f64 - t844 * t13645 / 16.0_f64 - t2408 * t13650 / 8.0_f64 - 7.0_f64 / 48.0_f64 * t12195 - 7.0_f64 / 96.0_f64 * t12199 + 3.0_f64 / 16.0_f64 * t2401 * t13656 + 7.0_f64 / 48.0_f64 * t12223 + t8659 * t13662 / 48.0_f64 + t1115 * t12111 / 16.0_f64 - t1115 * t9899 / 32.0_f64 + 3.0_f64 / 16.0_f64 * t1115 * t9820 + t335 * t13680 / 96.0_f64 + t2408 * t13684 / 8.0_f64 + 7.0_f64 / 96.0_f64 * t12246 + t13688 * t833 / 48.0_f64 + t3921 * t2503 / 32.0_f64 + 7.0_f64 / 48.0_f64 * t12253 - t335 * t13695 / 32.0_f64;
    (t13695, t13698)
}
