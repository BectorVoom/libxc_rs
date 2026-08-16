//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 862/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk862(t452: f64, t5710: f64, t6538: f64, t7165: f64, t979: f64, t1871: f64, t488: f64, t110: f64, t34384: f64, t34379: f64, t8411: f64, t7288: f64, t942: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t34737 = t452 * t5710 * t6538;
    let t34740 = t7165 * t979;
    let t34742 = t1871 * t488 * t34740;
    let t34746 = t1871 * t110 * t34384;
    let t34750 = t8411 * t110 * t34379;
    let t34754 = t452 * t7288 * t942;
    (t34737, t34740, t34742, t34746, t34750, t34754)
}
