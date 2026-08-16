//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2247/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2247<F: Float>(t30138: F, t7002: F, t13426: F, t7741: F, t18227: F, t28042: F, t4248: F, t108710: F, t1936: F, t21881: F, t93: F, t30143: F) -> (F, F, F, F, F, F, F) {
    let t109233 = F::cast_from(4.0_f64) * t30138 * t7002;
    let t109235 = F::cast_from(4.0_f64) * t13426 * t7741;
    let t109237 = F::cast_from(4.0_f64) * t18227 * t7741;
    let t109239 = F::cast_from(4.0_f64) * t4248 * t28042;
    let t109241 = F::cast_from(2.0_f64) * t108710 * t1936;
    let t109242 = t93 * t21881;
    let t109244 = F::cast_from(2.0_f64) * t109242 * t1936;
    let t109246 = F::cast_from(2.0_f64) * t30143 * t7002;
    (t109233, t109235, t109237, t109239, t109241, t109244, t109246)
}
