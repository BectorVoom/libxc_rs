//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1291/1306 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1291(t124169: f64, t125948: f64, t125950: f64, t128898: f64, t129370: f64, t130932: f64, t13426: f64, t1519: f64, t18227: f64, t2108: f64, t2163: f64, t27060: f64, t28652: f64, t33287: f64, t33306: f64, t34399: f64, t4248: f64, t4257: f64, t7537: f64, t7683: f64, t7969: f64, t7988: f64, t8892: f64) -> f64 {
    let t131037 = -2.0_f64 * t124169 * t1519 + t129370 * t2108 - 2.0_f64 * t130932 * t1519 - 2.0_f64 * t13426 * t8892 - 2.0_f64 * t18227 * t8892 - t2163 * t28652 - 2.0_f64 * t27060 * t7988 - 2.0_f64 * t33287 * t4257 - 2.0_f64 * t33306 * t4248 + t34399 * t7537 - t7683 * t7969 - t125948 - t125950 - t128898;
    t131037
}
