//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta200 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk841;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta200(t11717: f64, t3503: f64, t11713: f64, t1210: f64, t11153: f64, t3439: f64, t11147: f64, t11545: f64, t3247: f64, t415: f64, t61: f64, t121: f64, t3584: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t11727, t11728, t11737, t11738, t11759, t11764, t11778, t11779, t11784) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk841(t11717, t3503, t11713, t1210, t11153, t3439, t11147, t11545, t3247, t415, t61, t121, t3584);
    (t11727, t11728, t11737, t11738, t11759, t11764, t11778, t11779, t11784)
}
