//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 691/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk691<F: Float>(t508: F, t7373: F, t2089: F, t670: F, t2061: F, t212: F, t780: F, t689: F, t2062: F, t786: F, t789: F, t7023: F) -> (F, F, F, F, F, F, F, F) {
    let t7374 = t508 * t7373;
    let t7378 = t2089 * t670;
    let t7384 = t212 * t2061;
    let t7385 = t7384 * t780;
    let t7387 = F::cast_from(0.54878743191129263322e-2_f64) * t689 * t7385;
    let t7388 = t786 * t2062;
    let t7390 = F::cast_from(0.9757440539382783019e-2_f64) * t7388 * t789;
    let t7391 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t7023;
    (t7374, t7378, t7384, t7385, t7387, t7388, t7390, t7391)
}
