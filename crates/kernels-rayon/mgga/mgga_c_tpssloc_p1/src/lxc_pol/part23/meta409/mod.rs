//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta409 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1223;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1224;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta409(t1210: f64, t3500: f64, t65539: f64, t15734: f64, t5005: f64, t11818: f64, t248: f64, t3506: f64, t6225: f64, t3540: f64, t6170: f64, t6158: f64, t15730: f64, t5002: f64, t5024: f64, t3515: f64, t6230: f64, t11789: f64, t1227: f64, t5979: f64, t6165: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t65545, t65552, t65558, t65581, t65600) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1223(t1210, t3500, t65539, t15734, t5005, t11818, t248, t3506, t6225, t3540, t6170, t6158);
        let (t65605, t65628, t65632, t65647, t65664) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1224(t15730, t5002, t15734, t5024, t11818, t248, t3515, t6230, t11789, t1227, t5979, t3540, t6165);
    (t65545, t65552, t65558, t65581, t65600, t65605, t65628, t65632, t65647, t65664)
}
