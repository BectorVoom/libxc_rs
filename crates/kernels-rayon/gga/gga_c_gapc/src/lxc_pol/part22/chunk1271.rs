//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1271/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1271(t35175: f64, t3698: f64, t3702: f64, t116: f64, t198: f64, t22118: f64, t34195: f64, t34197: f64, t11347: f64, t3091: f64, t3670: f64, t9356: f64) -> (f64, f64, f64, f64) {
    let t35177 = t35175 * t3698 * t3702;
    let t35182 = t116 * t34195 * t34197 * t198 * t22118;
    let t35184 = t11347 * t3091;
    let t35186 = t3670 * t9356;
    (t35177, t35182, t35184, t35186)
}
