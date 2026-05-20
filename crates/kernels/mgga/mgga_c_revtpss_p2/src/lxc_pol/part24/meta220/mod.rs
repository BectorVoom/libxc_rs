//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta220 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk969;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk970;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk971;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta220<F: Float>(t1086: F, t3057: F, t3090: F, t11200: F, t225: F, t366: F, t2434: F, t371: F, t373: F, t367: F, t1065: F, t675: F, t1035: F, t11239: F, t342: F, t3145: F, t334: F, t11249: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11926, t11927, t11940) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk969::<F>(t1086, t3057, t3090, t11200, t225);
        let (t11941, t11970, t11972, t11986, t12046, t12047, t12050) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk970::<F>(t11940, t366, t2434, t371, t373, t367, t1065, t675, t1035, t11239, t342, t3145, t334);
        let t12051 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk971::<F>(t11249, t12050);
    (t11926, t11927, t11940, t11941, t11970, t11972, t11986, t12046, t12047, t12050, t12051)
}
