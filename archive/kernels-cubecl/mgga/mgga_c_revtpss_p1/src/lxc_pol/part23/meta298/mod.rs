//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta298 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1545;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1546;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1547;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1548;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta298<F: Float>(t3154: F, t999: F, t1086: F, t3046: F, t3090: F, t3316: F, t994: F, t4891: F, t1016: F, t697: F, t1011: F, t11132: F, t126: F, t373: F, t828: F, t3057: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t11860, t11865, t11866, t11874, t11875) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1545::<F>(t3154, t999, t1086, t3046, t3090, t3316, t994, t4891);
        let (t11881, t11890, t11921) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1546::<F>(t1016, t697, t1011, t11132, t126, t373);
        let t11922 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1547::<F>(t11921, t828);
        let (t11926, t11927) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1548::<F>(t1086, t3057, t3090);
    (t11860, t11865, t11866, t11874, t11875, t11881, t11890, t11921, t11922, t11926, t11927)
}
