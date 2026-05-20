//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta45 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk308;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk309;
use chunk2::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk310;
use chunk3::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk311;
use chunk4::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk312;
use chunk5::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk313;
use chunk6::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk314;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta45<F: Float>(t1032: F, t342: F, t358: F, t360: F, t336: F, t368: F, t365: F, t246: F, t372: F, t357: F, t73: F, t127: F, t371: F, t373: F, t367: F, t369: F, t361: F, t351: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t1033, t1034, t1035) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk308::<F>(t1032, t342, t358);
        let (t1036, t1038) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk309::<F>(t1035, t360, t336, t368);
        let (t1040, t1041) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk310::<F>(t1038, t365, t1036, t1033);
        let t1042 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk311::<F>(t246, t372);
        let t1045 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk312::<F>(t357, t73);
        let (t1058, t1060, t1062) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk313::<F>(t127, t371, t373, t367, t365, t369, t361);
        let t1063 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk314::<F>(t1062, t351);
    (t1034, t1035, t1036, t1038, t1040, t1041, t1042, t1045, t1058, t1060, t1062, t1063)
}
