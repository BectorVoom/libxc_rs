//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta402 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1211;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1212;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta402(t10224: f64, t5828: f64, t973: f64, t42875: f64, t5817: f64, t10508: f64, t248: f64, t3130: f64, t5873: f64, t3030: f64, t5848: f64, t3032: f64, t3129: f64, t3038: f64, t1041: f64, t10868: f64, t5685: f64, t18086: f64, t3069: f64, t10482: f64, t5872: f64, t5681: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t61597, t61600, t61663, t61734, t61735) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1211(t10224, t5828, t973, t42875, t5817, t10508, t248, t3130, t5873, t3030, t5848, t3032);
        let (t61736, t61739, t61782, t61950, t62079, t62137) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1212(t3129, t61735, t3038, t1041, t10868, t248, t5685, t18086, t3069, t10482, t5872, t5681);
    (t61597, t61600, t61663, t61734, t61736, t61739, t61782, t61950, t62079, t62137)
}
