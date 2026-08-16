//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta261 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk924;
use chunk1::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk925;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta261(t5234: f64, t5245: f64, t12283: f64, t6396: f64, t1362: f64, t19815: f64, t3799: f64, t6417: f64, t6422: f64, t16336: f64, t1831: f64, t3866: f64, t6427: f64, t6431: f64, t120: f64, t6414: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t19876, t19879, t19904, t19915, t19917, t19933, t19940) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk924(t5234, t5245, t12283, t6396, t1362, t19815, t3799, t6417, t6422, t16336, t1831, t3866, t6427);
        let (t19942, t19956) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk925(t3866, t6431, t120, t6414);
    (t19876, t19879, t19904, t19915, t19917, t19933, t19940, t19942, t19956)
}
