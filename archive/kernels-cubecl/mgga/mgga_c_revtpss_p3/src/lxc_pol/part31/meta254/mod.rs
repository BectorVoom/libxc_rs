//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta254 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1123;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1124;
use chunk2::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1125;
use chunk3::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1126;
use chunk4::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1127;
use chunk5::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1128;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta254<F: Float>(t1868: F, t5532: F, t3854: F, t3859: F, t3862: F, t3865: F, t3867: F, t3871: F, t3873: F, t4027: F, t4035: F, t4037: F, t4042: F, t4139: F, t6827: F, t6828: F, t6929: F, t118: F, t1502: F, t1519: F, t1843: F, t1847: F, t1911: F, t4248: F, t508: F, t511: F, t569: F, t5877: F, t5884: F, t5887: F, t5921: F, t651: F, t6765: F, t6773: F, t3: F, param_d: F, t116: F, t5883: F, t117: F, t5920: F, t1916: F, t1918: F, t572: F, t573: F, t2242: F, t38: F, t1925: F, t2247: F, t644: F, t84: F, t77: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let t6933 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1123::<F>(t1868, t5532, t3854, t3859, t3862, t3865, t3867, t3871, t3873, t4027, t4035, t4037, t4042, t4139, t6827, t6828);
        let (t6934, t6936) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1124::<F>(t6929, t6933, t118, t1502, t1519, t1843, t1847, t1911, t4248, t508, t511, t569, t5877, t5884, t5887, t5921, t651, t6765, t6773);
        let (t6937, t6941) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1125::<F>(t3, t6936, param_d);
        let (t6945, t6948, t6951, t6954, t6957) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1126::<F>(t116, t5883, t117, t5920, t1916, t1918, t572, t573, t6941, t2242, t38, t1925);
        let t6958 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1127::<F>(t2247, t6957);
        let t6960 = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1128::<F>(t644, t84, t77);
    (t6934, t6936, t6937, t6941, t6945, t6948, t6951, t6954, t6957, t6958, t6960)
}
