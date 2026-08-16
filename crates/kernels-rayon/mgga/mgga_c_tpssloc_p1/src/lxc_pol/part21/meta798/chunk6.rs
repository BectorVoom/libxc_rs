//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2778/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2778(t16969: f64, t9638: f64, t13258: f64, t16928: f64, t41385: f64, t5587: f64, t16673: f64, t2629: f64, t58181: f64, t842: f64, t13173: f64, t13177: f64, t13222: f64, t13231: f64, t13262: f64, t16836: f64, t16872: f64, t16985: f64, t20981: f64, t2379: f64, t2623: f64, t2635: f64, t2643: f64, t2681: f64, t40971: f64, t41096: f64, t4167: f64, t4178: f64, t4236: f64, t47012: f64, t47027: f64, t47262: f64, t47285: f64, t5527: f64, t5591: f64, t5628: f64, t58139: f64, t820: f64, t843: f64, t847: f64, t849: f64, t9990: f64) -> f64 {
    let t58791 = t9638 * t16969;
    let t58797 = t13258 * t16928;
    let t58809 = t41385 * t5587;
    let t58811 = t16673 * t2629;
    let t58834 = t58181 * t842;
    let t58837 = -7.0_f64 / 288.0_f64 * t58791 + t2643 * t13222 * t47262 * t5591 / 384.0_f64 + 7.0_f64 / 144.0_f64 * t58797 - t4178 * t13222 * t47262 * t20981 / 64.0_f64 + t13262 * t13222 * t47285 * t47012 / 64.0_f64 - t16836 * t13231 / 96.0_f64 + t41096 + 119.0_f64 / 6912.0_f64 * t58809 + t58811 * t2635 / 1536.0_f64 + 35.0_f64 / 128.0_f64 * t843 * t40971 * t820 * t5527 * t2379 - t13177 * t4236 / 768.0_f64 - t4167 * t13173 / 1536.0_f64 - t16872 * t2681 / 3072.0_f64 + 7.0_f64 / 288.0_f64 * t47027 - t9990 * t5628 / 768.0_f64 - t2623 * t16985 / 384.0_f64 - t843 * t847 * t820 * t58139 / 768.0_f64 - t58834 * t849 / 384.0_f64;
    t58837
}
