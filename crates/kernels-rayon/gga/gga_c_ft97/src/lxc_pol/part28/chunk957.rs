//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 957/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk957(t1317: f64, t1637: f64, t7252: f64, t1570: f64, t7211: f64, t1882: f64, t32352: f64, t32325: f64, t358: f64, t7165: f64, t637: f64, t7241: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t137218 = t1317 * t1637 * t7252;
    let t137219 = 2.0_f64 / 27.0_f64 * t137218;
    let t137224 = t7211 * t1570;
    let t137229 = t1882 * t32352;
    let t137231 = t32325 * t358;
    let t137236 = t7165 * t1570;
    let t137245 = t637 * t7241;
    (t137218, t137219, t137224, t137229, t137231, t137236, t137245)
}
