//! MGGA_C_TPSSLOC lxc pol kernel — _part23_v4rho4_4 meta441 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1285;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_meta441(t12283: f64, t20454: f64, t120: f64, t20489: f64, t16398: f64, t20475: f64, t20460: f64, t20565: f64, t3866: f64, t1827: f64, t57056: f64, t20492: f64, t39944: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t74110, t74120, t74147, t74189, t74191, t74212, t74214) = mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1285(t12283, t20454, t120, t20489, t16398, t20475, t20460, t20565, t3866, t1827, t57056, t20492, t39944);
    (t74110, t74120, t74147, t74189, t74191, t74212, t74214)
}
