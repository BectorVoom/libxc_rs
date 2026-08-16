//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 487/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk487(t143: f64, t160: f64, t7390: f64, t1384: f64, t5935: f64, t144: f64) -> (f64, f64, f64, f64) {
    let t7392 = t143 * t7390 * t160;
    let t7396 = t5935 * t1384;
    let t7397 = t144 * t7396;
    let t7400 = t1384 * t1384;
    (t7392, t7396, t7397, t7400)
}
