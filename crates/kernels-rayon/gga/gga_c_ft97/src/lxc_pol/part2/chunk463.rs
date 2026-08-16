//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 463/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk463(t2639: f64, t505: f64, t231: f64, t824: f64, t1526: f64, t2320: f64, t2638: f64, t342: f64, t343: f64, t830: f64, t829: f64, t10: f64, t1542: f64, t296: f64) -> (f64, f64, f64, f64, f64) {
    let t2640 = t2639 * t505;
    let t2644 = t231 * t824;
    let t2648 = t830 - t2638 - t1526 * t2320 * t2640 / 12.0_f64 - t342 * t343 * t2644 / 4.0_f64;
    let t2649 = t2648 * t829;
    let t2652 = t10 * t1542 * t296;
    (t2640, t2644, t2648, t2649, t2652)
}
