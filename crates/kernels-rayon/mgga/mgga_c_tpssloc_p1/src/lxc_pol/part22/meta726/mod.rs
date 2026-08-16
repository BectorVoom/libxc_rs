//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta726 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2379;
use chunk1::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2380;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta726(t48140: f64, t48143: f64, t68513: f64, t42444: f64, t20234: f64, t41687: f64, t607: f64, t10304: f64, t136: f64, t17151: f64, t3966: f64, t41880: f64, t68477: f64, t68498: f64, t68500: f64, t68502: f64, t68504: f64, t68506: f64, t68509: f64, t68511: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t68515, t68518, t68521, t68523, t68525, t68527, t68530) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2379(t48140, t48143, t68513, t42444, t20234, t41687, t607, t10304, t136, t17151, t3966, t41880, t68477);
        let t68532 = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2380(t68498, t68500, t68502, t68504, t68506, t68509, t68511, t68515, t68518, t68523, t68527, t68530);
    (t68515, t68518, t68521, t68523, t68525, t68527, t68530, t68532)
}
