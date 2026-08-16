//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta4 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk33;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk34;
use chunk2::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk35;
use chunk3::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk36;
use chunk4::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk37;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta4(t67: f64, t40: f64, t52: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let t68 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk33();
        let (t71, t72) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk34(t68, t67);
        let t73 = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk35(t40);
        let (t74, t75, t76) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk36(t40, t73, t52);
        let (t77, t78, t79, t80) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk37(t52, t76, t75, t72);
    (t68, t71, t72, t73, t74, t75, t76, t77, t78, t79, t80)
}
