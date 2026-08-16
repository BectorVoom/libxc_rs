//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2197/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2197(t16413: f64, t1985: f64, t214: f64, t225: f64, t567: f64, t22635: f64, t26214: f64, t26331: f64, t3734: f64, t22666: f64, t26202: f64, t22642: f64, t22643: f64, t7700: f64) -> (f64, f64, f64, f64) {
    let t90626 = t1985 * t214 * t16413 * t225 * t567;
    let t90634 = t26331 * t22635 * t26214 * t3734;
    let t90639 = t1985 * t22666 * t26202;
    let t90642 = t22642 * t22643 * t7700;
    (t90626, t90634, t90639, t90642)
}
