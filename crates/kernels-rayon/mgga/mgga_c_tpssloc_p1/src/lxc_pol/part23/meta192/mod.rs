//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta192 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk827;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk828;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta192(t11135: f64, t154: f64, t3584: f64, t3241: f64, t636: f64, t52: f64) -> (f64, f64, f64, f64, f64) {
        let (t11136, t11145, t11147) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk827(t11135, t154, t3584, t3241, t636);
        let (t11152, t11153) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk828(t3241, t52);
    (t11136, t11145, t11147, t11152, t11153)
}
