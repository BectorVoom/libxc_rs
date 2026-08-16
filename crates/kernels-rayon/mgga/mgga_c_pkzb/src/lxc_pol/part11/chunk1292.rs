//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1292/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1292(t8219: f64, t9847: f64, t8009: f64, t9850: f64, t11234: f64, t6142: f64, t851: f64, t2240: f64, t3069: f64, t3740: f64, t18427: f64, t18596: f64, t22230: f64, t22693: f64, t27262: f64, t27295: f64, t31067: f64, t31088: f64, t378: f64) -> (f64, f64, f64, f64, f64) {
    let t31456 = 18.0_f64 * t8219 * t9847;
    let t31458 = 12.0_f64 * t8009 * t9850;
    let t31461 = 24.0_f64 * t6142 * t11234 * t851;
    let t31464 = 18.0_f64 * t2240 * t3740 * t3069;
    let t31472 = (t18596 - 0.28842592592592592592e-1_f64 * t18427 - 0.86527777777777777779e-1_f64 * t22230 + t22693 + 0.37083333333333333333e-1_f64 * t27295 - 0.278125e-1_f64 * t27262 - 0.92708333333333333333e-2_f64 * t31067 + 0.278125e-1_f64 * t31088) * t378;
    (t31456, t31458, t31461, t31464, t31472)
}
