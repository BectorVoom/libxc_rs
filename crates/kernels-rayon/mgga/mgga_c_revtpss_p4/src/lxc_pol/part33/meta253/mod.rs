//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta253 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1126;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1127;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1128;
use chunk3::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1129;
use chunk4::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1130;
use chunk5::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1131;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta253(t1868: f64, t5532: f64, t3854: f64, t3859: f64, t3862: f64, t3865: f64, t3867: f64, t3871: f64, t3873: f64, t4027: f64, t4035: f64, t4037: f64, t4042: f64, t4139: f64, t6827: f64, t6828: f64, t6929: f64, t118: f64, t1502: f64, t1519: f64, t1843: f64, t1847: f64, t1911: f64, t4248: f64, t508: f64, t511: f64, t569: f64, t5877: f64, t5884: f64, t5887: f64, t5921: f64, t651: f64, t6765: f64, t6773: f64, t3: f64, param_d: f64, t116: f64, t5883: f64, t117: f64, t5920: f64, t1916: f64, t1918: f64, t572: f64, t573: f64, t2242: f64, t38: f64, t644: f64, t84: f64, t77: f64, t603: f64, t607: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t6933 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1126(t1868, t5532, t3854, t3859, t3862, t3865, t3867, t3871, t3873, t4027, t4035, t4037, t4042, t4139, t6827, t6828);
        let (t6934, t6936) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1127(t6929, t6933, t118, t1502, t1519, t1843, t1847, t1911, t4248, t508, t511, t569, t5877, t5884, t5887, t5921, t651, t6765, t6773);
        let (t6937, t6941) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1128(t3, t6936, param_d);
        let (t6945, t6948, t6951, t6954, t6959) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1129(t116, t5883, t117, t5920, t1916, t1918, t572, t573, t6941, t2242, t38, t644, t84);
        let t6960 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1130(t6959, t77);
        let t6963 = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1131(t603, t607);
    (t6934, t6936, t6937, t6941, t6945, t6948, t6951, t6954, t6960, t6963)
}
