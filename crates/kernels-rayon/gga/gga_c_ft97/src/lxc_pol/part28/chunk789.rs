//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 789/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk789(t165: f64, t7312: f64, t379: f64, t9073: f64, t7339: f64, t1969: f64, t5935: f64, t5968: f64, t604: f64, t7390: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32717 = t7312 * t165;
    let t32719 = t9073 * t32717 * t379;
    let t32722 = t7339 * t165;
    let t32723 = t32722 * t379;
    let t32724 = t1969 * t32723;
    let t32727 = t5935 * t5968;
    let t32729 = t7390 * t604;
    (t32717, t32719, t32722, t32724, t32727, t32729)
}
