//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta315 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1494;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1495;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta315(t15419: f64, t4724: f64, t3447: f64, t15026: f64, t3032: f64, t3514: f64, t3572: f64, t5002: f64, t3523: f64, t5005: f64, t5019: f64, t5024: f64, t11147: f64, t11778: f64, t3490: f64, t4993: f64, t248: f64, t3521: f64, t4733: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15420, t15422, t15437, t15438) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1494(t15419, t4724, t3447, t15026, t3032, t3514);
        let (t15446, t15448, t15450, t15452, t15453, t15484, t15486) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1495(t3572, t5002, t3523, t5005, t5019, t5024, t11147, t11778, t3490, t4993, t248, t3521, t4733);
    (t15420, t15422, t15437, t15438, t15446, t15448, t15450, t15452, t15453, t15484, t15486)
}
