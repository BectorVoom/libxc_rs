//! MGGA_C_TPSSLOC lxc pol kernel — _part22_v4rho4_3 meta281 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1430;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_meta281(t12945: f64, t707: f64, t3966: f64, t75: f64, t78: f64, t1484: f64, t212: f64, t9523: f64, t2586: f64, t213: f64, t4119: f64, t2570: f64, t67: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12946, t12950, t12961, t12984, t12985, t12986, t12988, t12997) = mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1430(t12945, t707, t3966, t75, t78, t1484, t212, t9523, t2586, t213, t4119, t2570, t67);
    (t12946, t12950, t12961, t12984, t12985, t12986, t12988, t12997)
}
