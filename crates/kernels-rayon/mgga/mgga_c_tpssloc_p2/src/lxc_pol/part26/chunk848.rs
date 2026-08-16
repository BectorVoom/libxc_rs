//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 26 (v4rho3sigma_2) CSE chunk 848/1384 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part26_v4rho3sigma_2_chunk848(t228: f64, t230: f64, t2667: f64, t2672: f64, t2675: f64, t4225: f64, t822: f64, t825: f64, t9938: f64, t9947: f64, t9951: f64, t9954: f64) -> f64 {
    let t9957 = 60.0_f64 * t228 * t9947 + 3.0_f64 * t228 * t9954 - t230 * t9938 + 9.0_f64 * t2667 * t825 - 36.0_f64 * t2672 * t822 + 9.0_f64 * t2675 * t822 - 36.0_f64 * t4225 * t9951;
    t9957
}
