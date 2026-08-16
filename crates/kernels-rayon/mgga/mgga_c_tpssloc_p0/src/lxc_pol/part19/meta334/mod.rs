//! MGGA_C_TPSSLOC lxc pol kernel — _part19_v4rho4_0 meta334 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1197;
use chunk1::mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1198;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_meta334(t40736: f64, t9467: f64, t9879: f64, t2374: f64, t39519: f64, t39503: f64, t118: f64, t2375: f64, t2448: f64, t39391: f64, t761: f64, t2427: f64, t9926: f64, t2531: f64, t9722: f64, t2379: f64, t39483: f64, t40727: f64, t40730: f64, t40732: f64, t40734: f64, t4314: f64, t9470: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40737, t40739, t40741, t40743, t40746, t40748, t40750) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1197(t40736, t9467, t9879, t2374, t39519, t39503, t118, t2375, t2448, t39391, t761, t2427, t9926);
        let (t40755, t40756) = mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1198(t2531, t9722, t2379, t39483, t40727, t40730, t40732, t40734, t40737, t40739, t40741, t40743, t40746, t40748, t40750, t4314, t9470);
    (t40737, t40739, t40741, t40743, t40746, t40748, t40750, t40755, t40756)
}
