//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta245 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1007;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1008;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta245(t1558: f64, t2811: f64, t2482: f64, t1531: f64, t37: f64, t1544: f64, t2475: f64, t124: f64, t136: f64, t243: f64, t220: f64, t10815: f64, t1561: f64, t10845: f64, t4430: f64, t853: f64, t4353: f64, t9794: f64, t10760: f64, t10890: f64, t1549: f64, t4416: f64, t808: f64, t10886: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14598, t14613, t14648, t14671, t14686, t14712) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1007(t1558, t2811, t2482, t1531, t37, t1544, t2475, t124, t136, t243, t220, t10815, t1561);
        let (t14716, t14718, t14761, t14765, t14779, t14780) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1008(t10845, t4430, t1558, t853, t4353, t9794, t10760, t10890, t1549, t4416, t808, t10886);
    (t14598, t14613, t14648, t14671, t14686, t14712, t14716, t14718, t14761, t14765, t14779, t14780)
}
