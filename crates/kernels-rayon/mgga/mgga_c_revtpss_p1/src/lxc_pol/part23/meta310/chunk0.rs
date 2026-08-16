//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1587/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1587(t1122: f64, t12879: f64, t247: f64, t1261: f64, t126: f64, t3617: f64, t1231: f64, t3655: f64, t2434: f64, t371: f64, t482: f64, t481: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12881 = t247 * t12879 * t1122;
    let t12882 = t1261 * t12881;
    let t12884 = t126 * t3617;
    let t12893 = t1231 * t3655;
    let t12898 = t371 * t2434 * t482;
    let t12900 = 0.63517063878621832551e-4_f64 * t481 * t12898;
    (t12881, t12882, t12884, t12893, t12898, t12900)
}
