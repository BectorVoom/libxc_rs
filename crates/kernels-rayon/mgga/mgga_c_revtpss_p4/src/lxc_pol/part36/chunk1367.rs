//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1367/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1367(t33: f64, t114200: f64, t116675: f64, t1469: f64, t2159: f64, t22671: f64, t30936: f64, t57: f64, t5825: f64, t8227: f64, t111696: f64, t113063: f64, t113065: f64, t113067: f64, t113076: f64, t113078: f64, t113084: f64, t113086: f64, t113089: f64, t113092: f64, t113095: f64, t116063: f64, t118: f64, t1519: f64, t18245: f64, t2163: f64, t2165: f64, t22633: f64, t23094: f64, t29427: f64, t30951: f64, t30963: f64, t4248: f64, t5887: f64, t5921: f64, t651: f64, t6934: f64, t7732: f64, t8158: f64, t8237: f64, dens_threshold: f64, rho1: f64, zeta_threshold: f64) -> f64 {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t116685 = piecewise3(t400, t114200, t116675 * t57 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t30936 * t1469 - 3.0_f64 / 2.0_f64 * t8227 * t5825 - t2159 * t22671 / 2.0_f64);
    let t116702 = -t113063 - t113065 - t113067 - 6.0_f64 * t111696 * t1519 - 12.0_f64 * t29427 * t5887 - t118 * (t116063 + t116685) + t113076 - t113078 - t113084 - t113086 - t113089 - 2.0_f64 * t651 * t2163 * t22633 - 6.0_f64 * t7732 * t30951 - 6.0_f64 * t29427 * t5921 + 3.0_f64 * t8237 * t6934 + t2165 * t23094 + t113092 + t113095 - 6.0_f64 * t18245 * t8158 - 12.0_f64 * t4248 * t30963;
    t116702
}
