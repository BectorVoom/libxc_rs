//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1139/1274 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1139<F: Float>(t1025: F, t1028: F, t11644: F, t11649: F, t11783: F, t15651: F, t15656: F, t15662: F, t15668: F, t15671: F, t15675: F, t1665: F, t3208: F, t3211: F, t3220: F, t3224: F, t4854: F, t4858: F) -> (F,) {
    let t15676 = -0.19055119163586549765e-3 * t11644 + 0.14291339372689912324e-3 * t11649 - 0.21437009059034868486e-3 * t11783 * t1665 - 0.42874018118069736972e-3 * t3224 * t4854 - 0.21437009059034868486e-3 * t1025 * t15651 - 0.42874018118069736972e-3 * t15656 * t1028 - 0.21437009059034868486e-3 * t4858 * t3220 - t15662 + 0.22866142996303859718e-2 * t3211 * t4854 - t15668 + 0.42874018118069736972e-3 * t15671 * t3208 - t15675;
    (t15676,)
}
