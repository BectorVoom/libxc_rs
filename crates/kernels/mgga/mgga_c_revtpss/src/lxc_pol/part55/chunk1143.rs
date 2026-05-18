//! MGGA_C_REVTPSS lxc pol — lxc_pol part 55 (v4rho2sigma2_10) CSE chunk 1143/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part55_v4rho2sigma2_10_chunk1143<F: Float>(t121206: F, t121232: F, t121177: F, t1385: F, t240: F, t27: F, t119967: F, t121204: F, t13847: F, t1399: F, t121210: F, t2453: F, t8705: F) -> (F, F, F, F, F, F, F) {
    let t121233 = t121232 * t121206;
    let t121234 = F::new(0.150583822711895824e-3) * t121233;
    let t121235 = t121232 * t121177;
    let t121245 = t1385 * t27 * t240;
    let t121246 = t119967 * t121245;
    let t121248 = t13847 * t121204 * t1399;
    let t121249 = t121246 * t121248;
    let t121272 = t2453 * t8705 * t121210;
    (t121234, t121235, t121245, t121246, t121248, t121249, t121272)
}
