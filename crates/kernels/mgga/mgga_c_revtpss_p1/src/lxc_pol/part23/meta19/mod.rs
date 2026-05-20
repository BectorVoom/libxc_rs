//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta19 (260520-c91 hierarchical CSE).
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

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk147;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk148;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk149;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk150;
use chunk4::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk151;
use chunk5::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk152;
use chunk6::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk153;
use chunk7::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk154;
use chunk8::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk155;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta19<F: Float>(t359: F, t360: F, t39: F, t40: F, rho0: F, t351: F, t335: F, t72: F, t245: F, t125: F, t66: F, t283: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t361, t362, t365) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk147::<F>(t359, t360, t39, t40, rho0);
        let t366 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk148::<F>(t361, t365);
        let t367 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk149::<F>(t351, t366);
        let t368 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk150::<F>(t335);
        let t369 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk151::<F>(t368);
        let (t370, t371) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk152::<F>(t369, t72, t245);
        let t372 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk153::<F>(t125, t66);
        let t373 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk154::<F>(t283);
        let t375 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk155::<F>(t371, t372, t373);
    (t361, t362, t365, t366, t367, t368, t369, t370, t371, t372, t373, t375)
}
