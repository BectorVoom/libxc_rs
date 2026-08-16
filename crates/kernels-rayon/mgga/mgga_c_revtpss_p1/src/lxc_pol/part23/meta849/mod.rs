//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta849 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2732;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta849(t1285: f64, t70994: f64, t17384: f64, t17605: f64, t17448: f64, t17451: f64, t1121: f64, t6587: f64, t13148: f64, t70916: f64, t13142: f64, t12772: f64, t21218: f64, t3625: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t70995, t71009, t71020, t71029, t71036, t71039, t71047) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2732(t1285, t70994, t17384, t17605, t17448, t17451, t1121, t6587, t13148, t70916, t13142, t12772, t21218, t3625);
    (t70995, t71009, t71020, t71029, t71036, t71039, t71047)
}
