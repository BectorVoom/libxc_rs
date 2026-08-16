//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta537 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2077;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2078;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta537(t751: f64, t9288: f64, t9897: f64, t2244: f64, t2517: f64, t2658: f64, t39488: f64, t761: f64, t2531: f64, t9919: f64, t707: f64, t9258: f64, t9467: f64, t9879: f64, t2374: f64, t39519: f64, t39503: f64, t118: f64, t2375: f64, t2448: f64, t39391: f64, t9722: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40726, t40729, t40732, t40733, t40736) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2077(t751, t9288, t9897, t2244, t2517, t2658, t39488, t761, t2531, t9919, t707, t9258);
        let (t40738, t40741, t40743, t40745, t40748, t40754) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2078(t9467, t9879, t2374, t39519, t39503, t118, t2375, t2448, t39391, t761, t2531, t9722);
    (t40726, t40729, t40732, t40733, t40736, t40738, t40741, t40743, t40745, t40748, t40754)
}
