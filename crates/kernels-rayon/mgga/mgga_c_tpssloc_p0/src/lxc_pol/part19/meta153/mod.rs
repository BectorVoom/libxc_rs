//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta153 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk764;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta153(t93: f64, t101: f64, t584: f64, t16: f64, t2: f64) -> (f64, f64, f64, f64) {
        let (t9108, t9174, t9211, t9212) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk764(t93, t101, t584, t16, t2);
    (t9108, t9174, t9211, t9212)
}
