//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1222;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1223;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta356<F: Float>(t1012: F, t24016: F, t23598: F, t373: F, t371: F, t372: F, t1651: F, t6244: F, t1011: F, t1025: F, t11859: F, t11875: F, t11941: F, t15671: F, t15926: F, t16220: F, t1665: F, t19773: F, t20005: F, t20017: F, t20021: F, t20025: F, t20030: F, t20034: F, t20051: F, t20055: F, t23994: F, t23999: F, t24009: F, t24013: F, t3115: F, t4858: F, t6273: F, t6278: F, t6339: F, t23872: F, t23926: F, t23988: F) -> (F, F, F, F, F, F) {
        let (t24017, t24022, t24024, t24031) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1222::<F>(t1012, t24016, t23598, t373, t371, t372, t1651, t6244);
        let (t24032, t24034, t24040) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1223::<F>(t24031, t373, t371, t372, t1011, t1025, t11859, t11875, t11941, t15671, t15926, t16220, t1665, t19773, t20005, t20017, t20021, t20025, t20030, t20034, t20051, t20055, t23994, t23999, t24009, t24013, t24017, t24024, t3115, t4858, t6273, t6278, t6339);
        let t24042 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1224::<F>(t23872, t23926, t23988, t24040);
    (t24022, t24024, t24031, t24032, t24034, t24042)
}
