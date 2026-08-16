//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1185/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1185(t1349: f64, t35027: f64, t376: f64, t1362: f64, t148238: f64, t148860: f64, t148897: f64, t148906: f64, t148960: f64, t149058: f64, t149549: f64, t149593: f64, t2: f64, t26: f64, t26780: f64, t27423: f64, t27429: f64, t32714: f64, t35222: f64, t4: f64, t564: f64, t7309: f64) -> f64 {
    let t149601 = t1349 * t376 * t35027;
    let t149607 = t32714 * t27423 / 9.0_f64 - t32714 * t27429 / 27.0_f64 - 2.0_f64 * t148238 - t564 * t35222 - 2.0_f64 * t149058 - 2.0_f64 * t148860 - 2.0_f64 * t148960 + (t149549 + t149593) * t2 * t4 * t26 * t1362 / 6.0_f64 + t149601 / 9.0_f64 + t7309 * t26780 / 6.0_f64 - 4.0_f64 * t148897 - 2.0_f64 * t148906;
    t149607
}
