//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 499/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk499(t3386: f64, t603: f64, t1245: f64, t539: f64, t544: f64, t1244: f64, t591: f64) -> (f64, f64, f64, f64) {
    let t3387 = t3386 * t603;
    let t3389 = t539 * t1245;
    let t3391 = t544 * t1245;
    let t3399 = t1244 * t591;
    (t3387, t3389, t3391, t3399)
}
