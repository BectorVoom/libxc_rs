//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 836/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk836(t12574: f64, t12577: f64, t12580: f64, t12584: f64, t12589: f64, t12592: f64, t12921: f64, t12925: f64, t12928: f64, t13123: f64, t9390: f64, t13104: f64, t13114: f64, t13122: f64) -> f64 {
    let t13133 = -t13123 + 3.0_f64 / 8.0_f64 * t12921 - t12925 / 2.0_f64 - t12928 / 4.0_f64 + 2.0_f64 / 3.0_f64 * t12574 + 8.0_f64 / 3.0_f64 * t12577 - 2.0_f64 / 9.0_f64 * t12580 + 2.0_f64 * t12584 - 6.0_f64 * t12589 + 4.0_f64 / 9.0_f64 * t12592 - t9390;
    let t13135 = t13104 + t13114 + t13122 + t13133;
    t13135
}
