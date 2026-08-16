//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1419/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1419(t1472: f64, t1484: f64, t17787: f64, t17838: f64, t34813: f64, t34816: f64, t44909: f64, t5203: f64, t53152: f64, t53155: f64, t59088: f64, t59152: f64, t59154: f64, t59160: f64, t59162: f64, t59165: f64, t59169: f64, t59171: f64, t59173: f64, t59176: f64, t59179: f64, t59181: f64) -> f64 {
    let t59379 = -t59088 - t59152 - t59154 - t59160 + 0.23392893589820816284e1_f64 * t53152 * t1484 + 4.0_f64 * t53155 * t1472 - t59162 + t59165 + t59169 - 0.70178680769462448852e1_f64 * t44909 * t5203 - 0.4155781415850207192e3_f64 * t34813 * t17787 + 0.82765347514623860983e4_f64 * t34816 * t17838 + t59171 + t59173 - t59176 - t59179 - t59181;
    t59379
}
