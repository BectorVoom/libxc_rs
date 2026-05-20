//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3647/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3647<F: Float>(t12571: F, t6552: F, t43995: F, t68253: F, t68255: F, t68257: F, t68262: F, t68267: F, t68271: F, t68275: F, t68277: F, t68282: F, t68287: F, t68292: F) -> (F, F) {
    let t68971 = F::cast_from(0.5848223622634646207e0_f64) * t12571 * t6552;
    let t68983 = F::cast_from(0.37083333333333333334e-1_f64) * t68253 + F::cast_from(0.41203703703703703704e-2_f64) * t68255 - F::cast_from(0.27469135802469135803e-2_f64) * t68257 + t43995 - F::cast_from(0.68672839506172839506e-2_f64) * t68262 + F::cast_from(0.10300925925925925926e-1_f64) * t68267 + F::new(0.2225e0) * t68271 + F::cast_from(0.37083333333333333334e-1_f64) * t68275 - F::cast_from(0.12361111111111111111e-1_f64) * t68277 - F::cast_from(0.12361111111111111111e-1_f64) * t68282 - F::cast_from(0.61805555555555555555e-2_f64) * t68287 - F::cast_from(0.37083333333333333333e-1_f64) * t68292;
    (t68971, t68983)
}
