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

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

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
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta141(t1100: f64, t389: f64, t1102: f64, t198: f64, t2868: f64, t2871: f64, t2878: f64, t2921: f64, t2929: f64, t3019: f64, t3021: f64, t3024: f64, t3028: f64, t3032: f64, t3036: f64, t3329: f64, t336: f64, t30: f64, t265: f64, t393: f64, t2838: f64, t1106: f64, t2257: f64, t2258: f64, t395: f64, t45: f64, t605: f64, t606: f64, t895: f64, dens_threshold: f64, rho0: f64, zeta_threshold: f64, t1941: f64, t268: f64, t404: f64, t1123: f64, t689: f64, t1263: f64, t159: f64, t635: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t3333, t3335, t3336) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk905(t1100, t389);
        let t3339 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk906(t1102, t198, t2868, t2871, t2878, t2921, t2929, t3019, t3021, t3024, t3028, t3032, t3036, t3329, t3333, t3336, t336);
        let (t3340, t3347) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk907(t30, t265, t393, t2838, t3339, t1106, t2257, t2258, t395, t45, t605, t606, t895, dens_threshold, rho0, zeta_threshold);
        let t3351 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk908(t2257);
        let t3356 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk909(t1941, t268, t404);
        let (t3357, t3358) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk910(t3356, t1123, t689);
        let t3360 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk911(t1263, t159);
        let t3361 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk912(t635);
        let t3362 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk913(t3361);
    (t3333, t3335, t3336, t3340, t3347, t3351, t3356, t3357, t3358, t3360, t3361, t3362)
}
