//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta617 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2063;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2064;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta617(t10778: f64, t1941: f64, t25222: f64, t4435: f64, t14868: f64, t2661: f64, t93082: f64, t14757: f64, t25234: f64, t14732: f64, t25245: f64, t14933: f64, t2482: f64, t25260: f64, t814: f64, t2689: f64, t27239: f64, t25277: f64, t4458: f64, t14685: f64, t14756: f64, t7021: f64, t14760: f64, t93015: f64, t27316: f64, t686: f64, t72: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t99062, t99066, t99070, t99074, t99078, t99085) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2063(t10778, t1941, t25222, t4435, t14868, t2661, t93082, t14757, t25234, t14732, t25245, t14933, t2482, t25260, t814);
        let (t99086, t99091, t99100, t99103, t99113, t99125) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2064(t99085, t2689, t27239, t25277, t4458, t14685, t14756, t7021, t14760, t93015, t27316, t686, t72);
    (t99062, t99066, t99070, t99074, t99078, t99086, t99091, t99100, t99103, t99113, t99125)
}
