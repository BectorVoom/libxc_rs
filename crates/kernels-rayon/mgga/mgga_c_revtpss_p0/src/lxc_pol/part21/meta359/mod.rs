//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta359 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1715;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1716;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1717;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta359(t11902: f64, t225: f64, t366: f64, t1053: f64, t3196: f64, t11151: f64, t247: f64, t3182: f64, t3163: f64, t3172: f64, t3161: f64, t1017: f64, t1063: f64, t11855: f64, t11859: f64, t11862: f64, t11866: f64, t11871: f64, t11875: f64, t11877: f64, t11881: f64, t11883: f64, t11886: f64, t11888: f64, t3101: f64, t3115: f64, t3120: f64, t3188: f64, t375: f64, t126: f64, t373: f64, t828: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11903, t11904, t11907, t11913, t11916, t11917, t11919) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1715(t11902, t225, t366, t1053, t3196, t11151, t247, t3182, t3163, t3172, t3161, t1017, t1063, t11855, t11859, t11862, t11866, t11871, t11875, t11877, t11881, t11883, t11886, t11888, t3101, t3115, t3120, t3188, t375);
        let t11921 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1716(t126, t373);
        let t11922 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1717(t11921, t828);
    (t11903, t11904, t11907, t11913, t11916, t11917, t11919, t11921, t11922)
}
