//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta209 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk852;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta209(t12344: f64, t1336: f64, t241: f64, t67: f64, t6924: f64, t1339: f64, t2690: f64, t3788: f64, t835: f64, t1995: f64, t246: f64, t3700: f64, t570: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12345, t12351, t12364, t12365, t12384, t12385, t12418, t12419, t12461) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk852(t12344, t1336, t241, t67, t6924, t1339, t2690, t3788, t835, t1995, t246, t3700, t570);
    (t12345, t12351, t12364, t12365, t12384, t12385, t12418, t12419, t12461)
}
