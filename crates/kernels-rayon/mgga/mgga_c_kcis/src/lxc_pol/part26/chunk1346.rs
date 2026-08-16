//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1346/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1346(t3734: f64, t7296: f64, t27517: f64, t29479: f64, t22387: f64, t28624: f64, t8196: f64, t97784: f64, t5913: f64, t97801: f64, t585: f64, t59975: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t103010 = t3734 * t7296;
    let t103012 = t27517 * t29479;
    let t103014 = t28624 * t22387;
    let t103016 = t97784 * t8196;
    let t103018 = t97801 * t5913;
    let t103020 = t59975 * t585;
    (t103010, t103012, t103014, t103016, t103018, t103020)
}
