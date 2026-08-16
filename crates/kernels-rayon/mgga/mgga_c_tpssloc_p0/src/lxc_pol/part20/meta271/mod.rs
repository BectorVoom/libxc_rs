//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta271 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1431;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1432;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta271(t1023: f64, t2771: f64, t10408: f64, t10402: f64, t3200: f64, t3041: f64, t884: f64, t3071: f64, t2776: f64, t3051: f64, t820: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10409, t10410, t10413) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1431(t1023, t2771, t10408, t10402, t3200);
        let (t10414, t10415, t10418, t10419, t10422) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1432(t3041, t884, t3071, t1023, t2776, t3051, t820);
    (t10409, t10410, t10413, t10414, t10415, t10418, t10419, t10422)
}
