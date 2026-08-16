//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1200/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1200(t39494: f64, t761: f64, t152: f64, t185: f64, t39097: f64, t153: f64, t157: f64, t39842: f64, t10140: f64, t10143: f64, t2374: f64, t39354: f64) -> (f64, f64, f64, f64, f64) {
    let t40779 = 0.51947577317044391277e2_f64 * t761 * t39494;
    let t40782 = 24.0_f64 * t39097 * t152 * t185;
    let t40784 = t153 * t157 * t39842;
    let t40785 = t10140 * t10143;
    let t40790 = 0.21687162600603479684e-1_f64 * t2374 * t39354;
    (t40779, t40782, t40784, t40785, t40790)
}
