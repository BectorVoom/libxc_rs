//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 774/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk774(t1339: f64, t452: f64, t5617: f64, t487: f64, t7281: f64, t379: f64, t1909: f64, t32412: f64, t83: f64, t110: f64, t1871: f64, t32120: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t32512 = t452 * t1339 * t5617;
    let t32515 = t487 * t7281;
    let t32516 = t32515 * t379;
    let t32517 = t1909 * t32516;
    let t32520 = t83 * t32412;
    let t32524 = t1871 * t110 * t32120;
    (t32512, t32515, t32516, t32517, t32520, t32524)
}
