//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta607 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2104;
use chunk1::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2105;
use chunk2::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2106;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta607<F: Float>(t1883: F, t4077: F, t27902: F, t686: F, t72: F, t25878: F, t97732: F, t27840: F, t689: F, t94674: F, t94669: F, t26069: F, t97922: F, t28011: F, t7284: F, t7289: F, t14269: F, t25885: F, t25931: F, t27837: F, t28008: F, t7279: F, t7308: F, t94823: F, t94854: F, t94857: F, t94865: F, t94867: F, t10073: F, t25937: F, t7282: F, t7910: F, t25899: F, t97899: F, t25953: F, t27899: F, t25981: F, t5677: F, t820: F, t844: F) -> (F, F, F, F, F, F) {
        let (t98062, t98067, t98069, t98071, t98078, t98081, t98084) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2104::<F>(t1883, t4077, t27902, t686, t72, t25878, t97732, t27840, t689, t94674, t94669, t26069, t97922);
        let t98092 = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2105::<F>(t28011, t686, t72, t7284, t7289, t14269, t25885, t25931, t27837, t28008, t7279, t7308, t94823, t94854, t94857, t94865, t94867, t98062, t98069, t98071, t98078, t98081, t98084);
        let (t98099, t98101, t98104, t98108) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2106::<F>(t10073, t25937, t7282, t7910, t25899, t97899, t25953, t27899, t25981, t5677, t820, t844);
    (t98067, t98092, t98099, t98101, t98104, t98108)
}
