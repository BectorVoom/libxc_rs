//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta600 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2249;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2250;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2251;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta600(t24031: f64, t373: f64, t371: f64, t372: f64, t1011: f64, t1025: f64, t11859: f64, t11875: f64, t11941: f64, t15671: f64, t15926: f64, t16220: f64, t1665: f64, t19773: f64, t20005: f64, t20017: f64, t20021: f64, t20025: f64, t20030: f64, t20034: f64, t20051: f64, t20055: f64, t23994: f64, t23999: f64, t24009: f64, t24013: f64, t24017: f64, t24024: f64, t3115: f64, t4858: f64, t6273: f64, t6278: f64, t6339: f64, t23872: f64, t23926: f64, t23988: f64, t225: f64, t385: f64, t1695: f64, t6350: f64, t11121: f64, t23964: f64, t996: f64, t1082: f64, t23640: f64, t378: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24032, t24034, t24040) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2249(t24031, t373, t371, t372, t1011, t1025, t11859, t11875, t11941, t15671, t15926, t16220, t1665, t19773, t20005, t20017, t20021, t20025, t20030, t20034, t20051, t20055, t23994, t23999, t24009, t24013, t24017, t24024, t3115, t4858, t6273, t6278, t6339);
        let t24042 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2250(t23872, t23926, t23988, t24040);
        let (t24044, t24047, t24048, t24061, t24068, t24075, t24078) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2251(t225, t24042, t385, t1695, t6350, t11121, t23964, t996, t24031, t1082, t23640, t378);
    (t24032, t24034, t24042, t24044, t24047, t24048, t24061, t24068, t24075, t24078)
}
