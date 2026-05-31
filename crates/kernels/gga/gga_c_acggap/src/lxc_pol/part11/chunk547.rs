//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 547/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk547<F: Float>(t334: F, t986: F, t339: F, t366: F, t374: F, t1137: F, t1145: F, t3106: F, t3109: F, t3141: F, t3160: F, t19: F, t2066: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t3570 = t986 * t334;
    let t3571 = t3570 * t339;
    let t3573 = t986 * t366;
    let t3574 = t3573 * t374;
    let t3576 = t1137 * t1145;
    let t3579 = F::cast_from(0.10866666666666666667e1_f64) * t3106;
    let t3580 = F::cast_from(0.978e0_f64) * t3109;
    let t3588 = F::cast_from(0.38033333333333333333e1_f64) * t3141;
    let t3592 = F::cast_from(0.12225e1_f64) * t3160;
    let t3615 = t2066 * t19;
    (t3570, t3571, t3573, t3574, t3576, t3579, t3580, t3588, t3592, t3615)
}
