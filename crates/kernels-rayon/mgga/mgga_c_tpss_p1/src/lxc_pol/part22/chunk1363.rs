//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1363/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1363(t1656: f64, t18967: f64, t20155: f64, t219: f64, t1266: f64, t13047: f64, t13055: f64, t13109: f64, t1639: f64, t1657: f64, t18483: f64, t18490: f64, t18496: f64, t18499: f64, t18947: f64, t20157: f64, t20183: f64, t20190: f64, t20200: f64, t20206: f64, t20211: f64, t3366: f64, t3367: f64, t3384: f64, t3385: f64, t5739: f64, t5740: f64, t5921: f64, t5925: f64, t5930: f64, t60653: f64, t60778: f64, t62453: f64, t6419: f64, t6424: f64, t6425: f64, t65667: f64, t65818: f64) -> f64 {
    let t67061 = t18967 * t1656;
    let t67083 = t20155 * t219;
    let t67109 = 2.0_f64 * t18483 * t20211 + 12.0_f64 * t60653 * t67061 * t18499 - 6.0_f64 * t5739 * t18490 * t6419 * t3366 - 6.0_f64 * t5739 * t18490 * t6424 * t3384 - 4.0_f64 * t18496 * t18967 * t1639 * t18499 - t5921 * t13109 + 2.0_f64 * t65667 * t5930 + 2.0_f64 * t5921 * t13055 - t62453 * t1657 - 2.0_f64 * t67083 * t1266 + 2.0_f64 * t60778 * t6425 + 4.0_f64 * t18483 * t20206 + 8.0_f64 * t18496 * t20190 * t1639 * t65818 + 2.0_f64 * t20157 * t3367 + 2.0_f64 * t18483 * t20200 + 2.0_f64 * t5739 * t5740 * t18947 * t1656 + 4.0_f64 * t18483 * t20183 - t20157 * t3385 - 6.0_f64 * t5921 * t13047 + 4.0_f64 * t65667 * t5925;
    t67109
}
