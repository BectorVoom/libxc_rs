//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta938 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3173;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta938(t12227: f64, t1732: f64, t12248: f64, t3433: f64, t16831: f64, t300: f64, t12429: f64, t1744: f64, t12472: f64, t5142: f64, t17150: f64, t3523: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t57795, t57818, t57854, t57861, t57944, t57972, t58000) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3173(t12227, t1732, t12248, t3433, t16831, t300, t12429, t1744, t12472, t5142, t17150, t3523);
    (t57795, t57818, t57854, t57861, t57944, t57972, t58000)
}
