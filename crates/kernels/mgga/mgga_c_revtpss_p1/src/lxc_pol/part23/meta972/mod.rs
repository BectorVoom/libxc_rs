//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta972 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3291;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3292;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3293;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3294;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3295;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3296;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3297;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta972<F: Float>(t14239: F, t22336: F, t13790: F, t6843: F, t10022: F, t2782: F, t22252: F, t4003: F, t46463: F, t48004: F, t48005: F, t48009: F, t5735: F, t5745: F, t74949: F, t74979: F, t74985: F, t74990: F, t1882: F, t6888: F, t22857: F, t555: F, t1399: F, t46505: F, t5675: F, t5755: F, t75021: F, t75024: F, t75026: F, t75035: F, t75039: F, t75041: F, t75049: F, t75053: F, t22953: F, t22954: F, t4101: F, t686: F, t72: F, t1892: F, t6861: F, t14193: F, t1883: F, t21990: F, t22005: F, t22016: F, t46515: F, t46518: F, t48080: F, t48082: F, t48090: F, t74965: F, t75060: F, t14122: F, t21981: F, t22858: F, t23037: F, t46526: F, t46554: F, t49167: F, t49439: F, t5659: F, t75068: F, t75071: F, t75074: F, t820: F, t85614: F, t22009: F, t46570: F, t49199: F, t49203: F, t49210: F, t74973: F, t75113: F, t75119: F, t75123: F, t75128: F, t1385: F, t22964: F, t14230: F, t14255: F, t49238: F, t49256: F, t49274: F, t6844: F, t74886: F, t75141: F, t75145: F, t75147: F, t5741: F, t75251: F, t47348: F, t47351: F, t47352: F, t47381: F, t49290: F, t75174: F, t75176: F, t75179: F, t75190: F, t75205: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t86422 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3291::<F>(t14239, t22336, t13790, t6843, t10022, t2782, t22252, t4003, t46463, t48004, t48005, t48009, t5735, t5745, t74949, t74979, t74985, t74990);
        let (t86441, t86445, t86453) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3292::<F>(t1882, t6888, t22857, t555, t1399, t46505, t5675, t5745, t5755, t75021, t75024, t75026, t75035, t75039, t75041, t75049, t75053);
        let (t86455, t86470, t86474) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3293::<F>(t22953, t555, t22954, t4101, t686, t72, t1892, t6861, t14193, t1883, t21990, t22005, t22016, t46515, t46518, t48080, t48082, t48090, t5675, t5745, t5755, t74965, t75060);
        let t86498 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3294::<F>(t1399, t14122, t14193, t21981, t22016, t22858, t23037, t46526, t46554, t49167, t49439, t5659, t5745, t5755, t75068, t75071, t75074, t820, t85614, t86445, t86470);
        let (t86506, t86533) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3295::<F>(t1892, t6843, t1399, t1883, t22009, t46570, t49199, t49203, t49210, t5659, t5755, t74973, t75113, t75119, t75123, t75128, t86455);
        let t86556 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3296::<F>(t1385, t22964, t1399, t14230, t14255, t1883, t22009, t49238, t49256, t49274, t5675, t5745, t6844, t74886, t75141, t75145, t75147, t820, t86470);
        let t86567 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3297::<F>(t5741, t75251, t47348, t47351, t47352, t47381, t49290, t75174, t75176, t75179, t75190, t75205);
    (t86422, t86441, t86445, t86453, t86455, t86470, t86474, t86498, t86506, t86533, t86556, t86567)
}
