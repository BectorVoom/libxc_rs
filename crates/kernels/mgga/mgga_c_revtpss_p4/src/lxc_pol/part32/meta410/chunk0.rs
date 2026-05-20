//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1421/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1421<F: Float>(t6132: F, t698: F, t6135: F, t18946: F, t930: F, t141: F, t6138: F, t18942: F, t18937: F, t2908: F, t11134: F, t11366: F, t11479: F, t11480: F, t18948: F) -> (F, F, F, F, F, F, F) {
    let t19002 = t698 * t6132;
    let t19004 = t698 * t6135;
    let t19006 = t930 * t18946;
    let t19007 = t141 * t19006;
    let t19009 = t698 * t6138;
    let t19013 = t930 * t18942;
    let t19014 = t141 * t19013;
    let t19016 = t2908 * t18937;
    let t19017 = t141 * t19016;
    let t19019 = -F::new(0.301925e0) * t18948 - t11479 - t11480 + F::cast_from(0.18396666666666666667e-1_f64) * t19002 - F::new(0.11038e0) * t19004 - F::new(0.82785e-1) * t19007 + F::new(0.5519e-1) * t19009 - F::cast_from(0.13418888888888888889e0_f64) * t11134 - F::cast_from(0.91983333333333333333e-1_f64) * t11366 + F::new(0.16557e0) * t19014 - F::new(0.27595e-1) * t19017;
    (t19002, t19004, t19007, t19009, t19014, t19017, t19019)
}
