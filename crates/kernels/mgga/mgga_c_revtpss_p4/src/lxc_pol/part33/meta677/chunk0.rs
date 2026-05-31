//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2209/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2209<F: Float>(t1936: F, t85360: F, t18245: F, t7002: F, t109150: F, t109153: F, t30138: F, t13426: F, t7741: F, t18227: F, t28042: F, t4248: F) -> (F, F, F, F, F, F, F, F) {
    let t109224 = F::cast_from(2.0_f64) * t85360 * t1936;
    let t109226 = F::cast_from(2.0_f64) * t18245 * t7002;
    let t109228 = F::cast_from(4.0_f64) * t109150 * t1936;
    let t109230 = F::cast_from(4.0_f64) * t109153 * t1936;
    let t109233 = F::cast_from(4.0_f64) * t30138 * t7002;
    let t109235 = F::cast_from(4.0_f64) * t13426 * t7741;
    let t109237 = F::cast_from(4.0_f64) * t18227 * t7741;
    let t109239 = F::cast_from(4.0_f64) * t4248 * t28042;
    (t109224, t109226, t109228, t109230, t109233, t109235, t109237, t109239)
}
