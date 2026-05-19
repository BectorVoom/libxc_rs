//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 781/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk781<F: Float>(t164: F, t9432: F, t2538: F, t729: F, t2556: F, t9283: F, t9286: F, t9289: F, t9292: F, t9296: F, t9298: F, t9300: F, t9303: F) -> (F, F, F, F) {
    let t9433 = t164 * t9432;
    let t9434 = t2538 * t729;
    let t9435 = t9434 * t2556;
    let t9446 = -F::new(0.47063e1) * t9283 + F::cast_from(0.31375333333333333334e1_f64) * t9286 - F::cast_from(0.36604555555555555556e1_f64) * t9289 - F::cast_from(0.16068111111111111111e1_f64) * t9292 + F::cast_from(0.28051666666666666666e0_f64) * t9296 - F::cast_from(0.56103333333333333332e0_f64) * t9298 - F::cast_from(0.6545388888888888889e0_f64) * t9300 - F::cast_from(0.46308888888888888888e0_f64) * t9303;
    (t9433, t9434, t9435, t9446)
}
