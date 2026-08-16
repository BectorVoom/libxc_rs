//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta972 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3291;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3292;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3293;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3294;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3295;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3296;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3297;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta972(t14239: f64, t22336: f64, t13790: f64, t6843: f64, t10022: f64, t2782: f64, t22252: f64, t4003: f64, t46463: f64, t48004: f64, t48005: f64, t48009: f64, t5735: f64, t5745: f64, t74949: f64, t74979: f64, t74985: f64, t74990: f64, t1882: f64, t6888: f64, t22857: f64, t555: f64, t1399: f64, t46505: f64, t5675: f64, t5755: f64, t75021: f64, t75024: f64, t75026: f64, t75035: f64, t75039: f64, t75041: f64, t75049: f64, t75053: f64, t22953: f64, t22954: f64, t4101: f64, t686: f64, t72: f64, t1892: f64, t6861: f64, t14193: f64, t1883: f64, t21990: f64, t22005: f64, t22016: f64, t46515: f64, t46518: f64, t48080: f64, t48082: f64, t48090: f64, t74965: f64, t75060: f64, t14122: f64, t21981: f64, t22858: f64, t23037: f64, t46526: f64, t46554: f64, t49167: f64, t49439: f64, t5659: f64, t75068: f64, t75071: f64, t75074: f64, t820: f64, t85614: f64, t22009: f64, t46570: f64, t49199: f64, t49203: f64, t49210: f64, t74973: f64, t75113: f64, t75119: f64, t75123: f64, t75128: f64, t1385: f64, t22964: f64, t14230: f64, t14255: f64, t49238: f64, t49256: f64, t49274: f64, t6844: f64, t74886: f64, t75141: f64, t75145: f64, t75147: f64, t5741: f64, t75251: f64, t47348: f64, t47351: f64, t47352: f64, t47381: f64, t49290: f64, t75174: f64, t75176: f64, t75179: f64, t75190: f64, t75205: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t86422 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3291(t14239, t22336, t13790, t6843, t10022, t2782, t22252, t4003, t46463, t48004, t48005, t48009, t5735, t5745, t74949, t74979, t74985, t74990);
        let (t86441, t86445, t86453) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3292(t1882, t6888, t22857, t555, t1399, t46505, t5675, t5745, t5755, t75021, t75024, t75026, t75035, t75039, t75041, t75049, t75053);
        let (t86455, t86470, t86474) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3293(t22953, t555, t22954, t4101, t686, t72, t1892, t6861, t14193, t1883, t21990, t22005, t22016, t46515, t46518, t48080, t48082, t48090, t5675, t5745, t5755, t74965, t75060);
        let t86498 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3294(t1399, t14122, t14193, t21981, t22016, t22858, t23037, t46526, t46554, t49167, t49439, t5659, t5745, t5755, t75068, t75071, t75074, t820, t85614, t86445, t86470);
        let (t86506, t86533) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3295(t1892, t6843, t1399, t1883, t22009, t46570, t49199, t49203, t49210, t5659, t5755, t74973, t75113, t75119, t75123, t75128, t86455);
        let t86556 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3296(t1385, t22964, t1399, t14230, t14255, t1883, t22009, t49238, t49256, t49274, t5675, t5745, t6844, t74886, t75141, t75145, t75147, t820, t86470);
        let t86567 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3297(t5741, t75251, t47348, t47351, t47352, t47381, t49290, t75174, t75176, t75179, t75190, t75205);
    (t86422, t86441, t86445, t86453, t86455, t86470, t86474, t86498, t86506, t86533, t86556, t86567)
}
