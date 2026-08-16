//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2618/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2618(t18332: f64, t4889: f64, t11668: f64, t11734: f64, t1202: f64, t1216: f64, t15503: f64, t15740: f64, t1735: f64, t18211: f64, t18383: f64, t18387: f64, t18948: f64, t21762: f64, t22174: f64, t22275: f64, t3577: f64, t488: f64, t52615: f64, t6192: f64, t66500: f64, t66512: f64, t66515: f64, t66518: f64) -> f64 {
    let t73076 = t4889 * t18332;
    let t73078 = -t15740 * t18387 / 768.0_f64 - t15740 * t18383 / 1536.0_f64 - t15503 * t18948 / 48.0_f64 + t52615 * t6192 / 144.0_f64 - 11.0_f64 / 162.0_f64 * t66500 + 5.0_f64 / 768.0_f64 * t3577 * t11668 * t1735 * t18211 - 209.0_f64 / 2592.0_f64 * t1202 * t22174 * t488 - t11734 * t22275 / 1024.0_f64 + 5.0_f64 / 2304.0_f64 * t3577 * t11668 * t21762 * t1216 - t66512 / 768.0_f64 - t66515 / 256.0_f64 + t66518 / 1536.0_f64 - t73076 / 81.0_f64;
    t73078
}
