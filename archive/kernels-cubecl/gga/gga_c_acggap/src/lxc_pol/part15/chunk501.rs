//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 501/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk501<F: Float>(t2792: F, t75: F, t689: F, t83: F, t2775: F, t199: F, t775: F, t13: F, t30: F, t778: F, t2666: F, t27: F) -> (F, F, F, F, F) {
    let t2793 = t75 * t2792;
    let t2795 = F::cast_from(1.0_f64) / t689 / t83;
    let t2796 = t2775 * t2795;
    let t2800 = F::cast_from(1.0_f64) / t775 / t199;
    let t2801 = t13 * t2800;
    let t2803 = F::cast_from(1.0_f64) / t778 / t30;
    let t2804 = t2666 * t2803;
    let t2805 = t2801 * t2804;
    let t2806 = F::cast_from(0.51726012919273400301e3_f64) * t2805;
    let t2808 = F::cast_from(1.0_f64) / t775 / t27;
    (t2793, t2795, t2796, t2806, t2808)
}
