//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta155 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk767;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk768;
use chunk2::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk769;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta155(t24: f64, t9238: f64, t2241: f64, t645: f64, t2307: f64, t607: f64, t65: f64, t67: f64, t1864: f64, t2250: f64, t2244: f64, t628: f64, t584: f64, t9212: f64, t25: f64, t28: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t9239, t9240, t9243, t9247, t9248, t9251) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk767(t24, t9238, t2241, t645, t2307, t607, t65, t67, t1864, t2250, t2244, t628);
        let (t9256, t9257) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk768(t584, t9212);
        let t9258 = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk769(t25, t28, t9257, zeta_threshold);
    (t9239, t9240, t9243, t9247, t9248, t9251, t9256, t9257, t9258)
}
