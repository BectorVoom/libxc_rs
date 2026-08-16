//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta599 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2064;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2065;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta599(t23168: f64, t23223: f64, t1882: f64, t81686: f64, t9537: f64, t213: f64, t225: f64, t852: f64, t23164: f64, t23204: f64, t23222: f64, t23238: f64, t23196: f64, t6562: f64, t23202: f64, t6556: f64, t81632: f64, t23012: f64, t6573: f64, t1883: f64, t82045: f64, t6555: f64, t82133: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t82150, t82154, t82159, t82172, t82174) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2064(t23168, t23223, t1882, t81686, t9537, t213, t225, t852, t23164, t23204, t23222, t23238);
        let (t82182, t82197, t82209, t82211, t82219, t82221) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2065(t23196, t23204, t6562, t225, t23202, t6556, t81632, t23012, t6573, t1883, t82045, t23164, t6555, t82133);
    (t82150, t82154, t82159, t82172, t82174, t82182, t82197, t82209, t82211, t82219, t82221)
}
