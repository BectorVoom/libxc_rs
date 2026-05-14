//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1217/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1217<F: Float>(t33: F, t114200: F, t116675: F, t1469: F, t2159: F, t22671: F, t30936: F, t57: F, t5825: F, t8227: F, t111696: F, t113063: F, t113065: F, t113067: F, t113076: F, t113078: F, t113084: F, t113086: F, t113089: F, t113092: F, t113095: F, t116063: F, t118: F, t1519: F, t18245: F, t2163: F, t2165: F, t22633: F, t23094: F, t29427: F, t30951: F, t30963: F, t4248: F, t5887: F, t5921: F, t651: F, t6934: F, t7732: F, t8158: F, t8237: F, dens_threshold: F, rho1: F, zeta_threshold: F) -> (F,) {
    let t34 = t33 <= zeta_threshold;
    let t400 = rho1 <= dens_threshold || t34;
    let t116685 = piecewise3(t400, t114200, t116675 * t57 / 2.0 - 3.0 / 2.0 * t30936 * t1469 - 3.0 / 2.0 * t8227 * t5825 - t2159 * t22671 / 2.0);
    let t116702 = -t113063 - t113065 - t113067 - 6.0 * t111696 * t1519 - 12.0 * t29427 * t5887 - t118 * (t116063 + t116685) + t113076 - t113078 - t113084 - t113086 - t113089 - 2.0 * t651 * t2163 * t22633 - 6.0 * t7732 * t30951 - 6.0 * t29427 * t5921 + 3.0 * t8237 * t6934 + t2165 * t23094 + t113092 + t113095 - 6.0 * t18245 * t8158 - 12.0 * t4248 * t30963;
    (t116702,)
}
