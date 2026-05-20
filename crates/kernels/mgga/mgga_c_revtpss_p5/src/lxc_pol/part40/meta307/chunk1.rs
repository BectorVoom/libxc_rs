//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1079/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1079<F: Float>(t1033: F, t11267: F, t3169: F, t3173: F, t2866: F, t914: F, t2923: F, t910: F, t287: F, t2922: F, t275: F, t11132: F) -> (F, F, F, F, F, F) {
    let t11268 = t1033 * t11267;
    let t11271 = t3169 * t3173;
    let t11289 = t2866 * t914;
    let t11294 = t910 * t2923;
    let t11298 = F::new(1.0) / t2922 / t287;
    let t11299 = t275 * t11298;
    let t11304 = F::new(28.0) / F::new(27.0) * t11132;
    (t11268, t11271, t11289, t11294, t11299, t11304)
}
