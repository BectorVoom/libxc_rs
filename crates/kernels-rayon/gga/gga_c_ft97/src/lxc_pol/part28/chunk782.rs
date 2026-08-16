//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 782/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk782(t492: f64, t7211: f64, t452: f64, t488: f64, t432: f64, t7281: f64, t379: f64, t7229: f64, t8557: f64, t1901: f64, t32591: f64, t32594: f64, t32599: f64, t32603: f64, t32607: f64, t32610: f64, t32613: f64, t32617: f64, t32622: f64, t446: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t32625 = t7211 * t492;
    let t32627 = t452 * t488 * t32625;
    let t32630 = t7281 * t432;
    let t32632 = t452 * t488 * t32630;
    let t32635 = t7229 * t379;
    let t32636 = t8557 * t32635;
    let t32639 = 2.0_f64 / 3.0_f64 * t446 * t32591 + 2.0_f64 / 9.0_f64 * t1901 * t32594 + t1901 * t32599 / 9.0_f64 - 4.0_f64 / 3.0_f64 * t1901 * t32603 - 4.0_f64 / 3.0_f64 * t1901 * t32607 + 2.0_f64 / 9.0_f64 * t1901 * t32610 + 4.0_f64 / 3.0_f64 * t446 * t32613 + 2.0_f64 / 3.0_f64 * t446 * t32617 + 2.0_f64 / 3.0_f64 * t446 * t32622 + t446 * t32627 / 3.0_f64 + t446 * t32632 / 3.0_f64 - 2.0_f64 / 9.0_f64 * t1901 * t32636;
    (t32625, t32627, t32630, t32632, t32635, t32636, t32639)
}
