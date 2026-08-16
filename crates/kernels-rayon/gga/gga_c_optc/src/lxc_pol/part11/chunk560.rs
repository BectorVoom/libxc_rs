//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 560/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk560(t50: f64, t1896: f64, t4570: f64, t4573: f64, t52: f64, t4569: f64, t59: f64, zeta_threshold: f64) -> f64 {
    let t51 = t50 <= zeta_threshold;
    let t4577 = piecewise3(t51, 0.0_f64, 4.0_f64 / 9.0_f64 * t1896 * t4570 + 4.0_f64 / 3.0_f64 * t52 * t4573);
    let t4579 = (t4569 + t4577) * t59;
    t4579
}
