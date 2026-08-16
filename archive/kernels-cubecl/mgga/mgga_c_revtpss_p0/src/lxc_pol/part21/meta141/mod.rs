//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta141 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;
mod chunk7;
mod chunk8;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk905;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk906;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk907;
use chunk3::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk908;
use chunk4::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk909;
use chunk5::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk910;
use chunk6::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk911;
use chunk7::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk912;
use chunk8::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta141<F: Float>(t1100: F, t389: F, t1102: F, t198: F, t2868: F, t2871: F, t2878: F, t2921: F, t2929: F, t3019: F, t3021: F, t3024: F, t3028: F, t3032: F, t3036: F, t3329: F, t336: F, t30: F, t265: F, t393: F, t2838: F, t1106: F, t2257: F, t2258: F, t395: F, t45: F, t605: F, t606: F, t895: F, dens_threshold: F, rho0: F, zeta_threshold: F, t1941: F, t268: F, t404: F, t1123: F, t689: F, t1263: F, t159: F, t635: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t3333, t3335, t3336) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk905::<F>(t1100, t389);
        let t3339 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk906::<F>(t1102, t198, t2868, t2871, t2878, t2921, t2929, t3019, t3021, t3024, t3028, t3032, t3036, t3329, t3333, t3336, t336);
        let (t3340, t3347) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk907::<F>(t30, t265, t393, t2838, t3339, t1106, t2257, t2258, t395, t45, t605, t606, t895, dens_threshold, rho0, zeta_threshold);
        let t3351 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk908::<F>(t2257);
        let t3356 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk909::<F>(t1941, t268, t404);
        let (t3357, t3358) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk910::<F>(t3356, t1123, t689);
        let t3360 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk911::<F>(t1263, t159);
        let t3361 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk912::<F>(t635);
        let t3362 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk913::<F>(t3361);
    (t3333, t3335, t3336, t3340, t3347, t3351, t3356, t3357, t3358, t3360, t3361, t3362)
}
