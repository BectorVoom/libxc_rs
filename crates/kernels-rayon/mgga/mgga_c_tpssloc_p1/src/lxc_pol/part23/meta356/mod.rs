//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta356 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1153;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta356(t41654: f64, t10969: f64, t154: f64, t2769: f64, t2289: f64, t2903: f64, t2928: f64, t315: f64, t10213: f64, t241: f64, t270: f64, t276: f64, t39267: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t41655, t41664, t41666, t41687, t41741, t41825, t41826, t41880, t41904, t41935) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1153(t41654, t10969, t154, t2769, t2289, t2903, t2928, t315, t10213, t241, t270, t276, t39267);
    (t41655, t41664, t41666, t41687, t41741, t41825, t41826, t41880, t41904, t41935)
}
