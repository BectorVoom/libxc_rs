//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta359 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1715;
use chunk1::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1716;
use chunk2::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1717;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta359<F: Float>(t11902: F, t225: F, t366: F, t1053: F, t3196: F, t11151: F, t247: F, t3182: F, t3163: F, t3172: F, t3161: F, t1017: F, t1063: F, t11855: F, t11859: F, t11862: F, t11866: F, t11871: F, t11875: F, t11877: F, t11881: F, t11883: F, t11886: F, t11888: F, t3101: F, t3115: F, t3120: F, t3188: F, t375: F, t126: F, t373: F, t828: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t11903, t11904, t11907, t11913, t11916, t11917, t11919) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1715::<F>(t11902, t225, t366, t1053, t3196, t11151, t247, t3182, t3163, t3172, t3161, t1017, t1063, t11855, t11859, t11862, t11866, t11871, t11875, t11877, t11881, t11883, t11886, t11888, t3101, t3115, t3120, t3188, t375);
        let t11921 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1716::<F>(t126, t373);
        let t11922 = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1717::<F>(t11921, t828);
    (t11903, t11904, t11907, t11913, t11916, t11917, t11919, t11921, t11922)
}
