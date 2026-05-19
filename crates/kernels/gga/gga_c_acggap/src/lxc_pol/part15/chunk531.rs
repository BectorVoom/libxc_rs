//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 531/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk531<F: Float>(t1963: F, t22: F, t161: F, t151: F, t177: F, t334: F, t986: F, t339: F, t366: F, t374: F, t3106: F, t3109: F) -> (F, F, F, F, F, F, F, F) {
    let t3558 = F::new(1.0) / t22 / t1963;
    let t3559 = t161 * t3558;
    let t3562 = F::cast_from(0.37792653007779990369e-1_f64) * t151 * t3559 * t177;
    let t3570 = t986 * t334;
    let t3571 = t3570 * t339;
    let t3573 = t986 * t366;
    let t3574 = t3573 * t374;
    let t3579 = F::cast_from(0.10866666666666666667e1_f64) * t3106;
    let t3580 = F::new(0.978e0) * t3109;
    (t3558, t3562, t3570, t3571, t3573, t3574, t3579, t3580)
}
