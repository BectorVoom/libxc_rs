//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1910/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1910<F: Float>(t5: F, t28115: F, t28157: F, t117: F, t7239: F, t7898: F, t197: F, t530: F, t2013: F) -> (F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::cast_from(0.0_f64) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t28159 = piecewise3::<F>(t8, F::cast_from(0.0_f64), t28115 + t28157);
    let t28160 = t28159 * t117;
    let t28165 = F::cast_from(3.0_f64) * t7898 * t7239;
    let t28166 = t197 * t530;
    let t28167 = t2013 * t28166;
    (t28159, t28160, t28165, t28166, t28167)
}
