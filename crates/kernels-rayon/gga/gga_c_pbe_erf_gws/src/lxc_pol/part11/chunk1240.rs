//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1240/1302 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk1240(t49087: f64, t6659: f64, t858: f64, t884: f64, t3134: f64, t45320: f64, t11778: f64, t11794: f64, t11984: f64, t13243: f64, t45190: f64, t13403: f64, t2255: f64, t2277: f64, t2345: f64, t28923: f64, t3247: f64, t37257: f64, t3757: f64, t3772: f64, t44282: f64, t45192: f64, t45194: f64, t9441: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t49634 = 3.0_f64 / 2.0_f64 * t884 * t6659 * t858 * t49087;
    let t49641 = t45320 * t3134 / 12.0_f64;
    let t49643 = t11794 * t11778 / 16.0_f64;
    let t49650 = t11984 * t13243 / 4.0_f64;
    let t49652 = 7.0_f64 / 12.0_f64 * t45190;
    let t49655 = -t49634 - 3.0_f64 / 32.0_f64 * t3247 * t2345 * t44282 * t13403 + 119.0_f64 / 384.0_f64 * t37257 - t49641 - t49643 + t2277 * t2255 * t9441 * t3757 * t3772 / 256.0_f64 + t49650 - 595.0_f64 / 1296.0_f64 * t28923 + t49652 + 7.0_f64 / 48.0_f64 * t45192 - 35.0_f64 / 48.0_f64 * t45194;
    (t49634, t49641, t49643, t49650, t49652, t49655)
}
