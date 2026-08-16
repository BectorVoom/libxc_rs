//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 924/1184 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk924(t7021: f64, t870: f64, t28842: f64, t1495: f64, t2681: f64, t6353: f64, t848: f64, t108446: f64, t3766: f64, t27669: f64, t79528: f64, t226: f64, t27703: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t114578 = t870 * t7021;
    let t114751 = t28842 * t870;
    let t114820 = t2681 * t1495;
    let t114847 = t848 * t6353;
    let t122830 = t3766 * t108446;
    let t123028 = t79528 * t27669;
    let t123124 = t27703 * t226;
    (t114578, t114751, t114820, t114847, t122830, t123028, t123124)
}
