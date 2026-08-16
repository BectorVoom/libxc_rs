//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta115 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk596;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk597;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta115(t340: f64, t974: f64, t1604: f64, t225: f64, t1539: f64, t248: f64, t3051: f64, t1041: f64, t247: f64, t375: f64, t1043: f64, t2775: f64, t2770: f64, t3061: f64, t135: f64, t1606: f64, t973: f64, t1036: f64, t1612: f64, t1616: f64, t3101: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t4546, t4557, t4571, t4572, t4582) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk596(t340, t974, t1604, t225, t1539, t248, t3051, t1041, t247, t375);
        let (t4583, t4588, t4604, t4625, t4630) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk597(t1043, t2775, t2770, t3061, t135, t1606, t973, t1036, t1612, t1616, t248, t3101);
    (t4546, t4557, t4571, t4572, t4582, t4583, t4588, t4604, t4625, t4630)
}
