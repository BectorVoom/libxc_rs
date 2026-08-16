//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2648/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2648(t40018: f64, t5223: f64, t16379: f64, t40021: f64, t12156: f64, t12240: f64, t12353: f64, t12407: f64, t1307: f64, t1369: f64, t16225: f64, t16305: f64, t16306: f64, t16321: f64, t16355: f64, t1810: f64, t210: f64, t3733: f64, t3803: f64, t3876: f64, t39936: f64, t40025: f64, t5240: f64, t5246: f64, t53907: f64, t53910: f64, t53918: f64, t53920: f64, t53921: f64) -> f64 {
    let t53927 = t40018 * t5223;
    let t53928 = 35.0_f64 / 24.0_f64 * t53927;
    let t53929 = t40021 * t16379;
    let t53943 = 7.0_f64 / 192.0_f64 * t53907 - t53910 * t1369 / 256.0_f64 - t16321 * t3876 / 256.0_f64 - 5.0_f64 / 128.0_f64 * t5240 * t12353 - t53918 - t53920 - 7.0_f64 / 8.0_f64 * t53921 + 3.0_f64 / 16.0_f64 * t3733 * t210 * t16355 * t1307 + t53928 + 7.0_f64 / 4.0_f64 * t53929 + t39936 + 5.0_f64 / 4.0_f64 * t40025 * t210 * t1810 * t12156 - t5246 * t16305 * t16225 * t12240 / 128.0_f64 + t3803 * t16305 * t16306 * t12407 / 256.0_f64;
    t53943
}
