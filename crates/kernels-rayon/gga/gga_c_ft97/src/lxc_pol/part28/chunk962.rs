//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 962/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk962(t32355: f64, t378: f64, t1286: f64, t1637: f64, t7213: f64, t22892: f64, t7162: f64, t32365: f64, t487: f64, t1851: f64, t7264: f64, t137089: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t137525 = t378 * t32355;
    let t137531 = 2.0_f64 / 27.0_f64 * t1286 * t1637 * t7213;
    let t137547 = t7162 * t22892;
    let t137561 = t32365 * t487;
    let t137564 = t7264 * t1851;
    let t137623 = 10.0_f64 / 9.0_f64 * t137089;
    (t137525, t137531, t137547, t137561, t137564, t137623)
}
