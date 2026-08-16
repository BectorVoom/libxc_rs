//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta935 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3167;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3168;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta935(t1260: f64, t44843: f64, t17423: f64, t17426: f64, t343: f64, t56: f64, t816: f64, t13026: f64, t65: f64, t12256: f64, t12772: f64, t17634: f64, t3625: f64, t17395: f64, t3746: f64, t17689: f64, t44425: f64, t17435: f64, t3667: f64, t1235: f64, t127: f64, t17278: f64, t371: f64, t1256: f64, t17311: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t57520, t57534, t57548, t57550, t57569) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3167(t1260, t44843, t17423, t17426, t343, t56, t816, t13026, t65, t12256, t12772, t17634, t3625);
        let (t57571, t57584, t57586, t57590, t57602) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3168(t17395, t3746, t17689, t3625, t44425, t17435, t3667, t1235, t127, t17278, t371, t1256, t17311);
    (t57520, t57534, t57548, t57550, t57569, t57571, t57584, t57586, t57590, t57602)
}
