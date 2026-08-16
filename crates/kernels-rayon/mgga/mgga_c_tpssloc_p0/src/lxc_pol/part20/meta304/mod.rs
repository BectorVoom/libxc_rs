//! MGGA_C_TPSSLOC lxc pol kernel — _part20_v4rho4_1 meta304 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1547;
use chunk1::mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1548;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_meta304(t11191: f64, t3315: f64, t11190: f64, t11135: f64, t1102: f64, t3270: f64, t3279: f64, t3287: f64, t10292: f64, t281: f64, t415: f64, t1113: f64, t11163: f64, t136: f64, t11172: f64, t1114: f64, t2403: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11192, t11194, t11195, t11197, t11200, t11203, t11204, t11205) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1547(t11191, t3315, t11190, t11135, t1102, t3270, t3279, t3287, t10292, t281, t415, t1113, t11163);
        let (t11206, t11208, t11209, t11211) = mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1548(t11205, t136, t1113, t11172, t1114, t2403);
    (t11192, t11194, t11195, t11197, t11200, t11203, t11204, t11205, t11206, t11208, t11209, t11211)
}
