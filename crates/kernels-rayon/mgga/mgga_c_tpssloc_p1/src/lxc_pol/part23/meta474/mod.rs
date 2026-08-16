//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta474 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1418;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1419;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta474(t136: f64, t3297: f64, t78031: f64, t78039: f64, t1113: f64, t78047: f64, t78043: f64, t1100: f64, t78077: f64, t3287: f64, t78025: f64, t11219: f64, t78035: f64, t1661: f64, t71445: f64, t71448: f64, t18754: f64, t5999: f64, t18746: f64, t43895: f64, t63361: f64, t78057: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t78084, t78087, t78090, t78093, t78095, t78097, t78100) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1418(t136, t3297, t78031, t78039, t1113, t78047, t78043, t1100, t78077, t3287, t78025, t11219, t78035);
        let (t78103, t78105, t78107, t78109, t78112) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1419(t1661, t71445, t71448, t18754, t5999, t18746, t43895, t63361, t78057, t78084, t78087, t78090, t78093, t78095, t78097, t78100);
    (t78084, t78087, t78090, t78093, t78095, t78097, t78100, t78103, t78105, t78107, t78109, t78112)
}
