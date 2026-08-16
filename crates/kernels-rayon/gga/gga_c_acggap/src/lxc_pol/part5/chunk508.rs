//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 508/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk508(t2643: f64, t40: f64, t218: f64, t771: f64, t777: f64, t779: f64, t220: f64, t760: f64, t271: f64, t680: f64, t690: f64, t273: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t2644 = t40 * t2643;
    let t2654 = t777 * t771 * t779 * t218;
    let t2655 = 0.48245938496077605201e2_f64 * t2654;
    let t2657 = t760 * t220 * t771;
    let t2658 = 6.0_f64 * t2657;
    let t2660 = t680 * t690 * t271;
    let t2663 = t273 * t680;
    (t2644, t2654, t2655, t2657, t2658, t2660, t2663)
}
