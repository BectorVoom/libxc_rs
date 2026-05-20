//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2063;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta617<F: Float>(t10778: F, t1941: F, t25222: F, t4435: F, t14868: F, t2661: F, t93082: F, t14757: F, t25234: F, t14732: F, t25245: F, t14933: F, t2482: F, t25260: F, t814: F, t2689: F, t27239: F, t25277: F, t4458: F, t14685: F, t14756: F, t7021: F, t14760: F, t93015: F, t27316: F, t686: F, t72: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t99062, t99066, t99070, t99074, t99078, t99085) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2063::<F>(t10778, t1941, t25222, t4435, t14868, t2661, t93082, t14757, t25234, t14732, t25245, t14933, t2482, t25260, t814);
        let (t99086, t99091, t99100, t99103, t99113, t99125) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2064::<F>(t99085, t2689, t27239, t25277, t4458, t14685, t14756, t7021, t14760, t93015, t27316, t686, t72);
    (t99062, t99066, t99070, t99074, t99078, t99086, t99091, t99100, t99103, t99113, t99125)
}
