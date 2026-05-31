//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2248/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2248<F: Float>(t27123: F, t7741: F, t28219: F, t28042: F, t7889: F, t2322: F, t30004: F, t5523: F, t105850: F, t109006: F, t109233: F, t109235: F, t109237: F, t109239: F, t109241: F, t109244: F, t109246: F) -> F {
    let t109248 = F::cast_from(4.0_f64) * t27123 * t7741;
    let t109250 = F::cast_from(4.0_f64) * t28219 * t7741;
    let t109252 = F::cast_from(4.0_f64) * t7889 * t28042;
    let t109254 = F::cast_from(2.0_f64) * t2322 * t30004;
    let t109256 = F::cast_from(2.0_f64) * t5523 * t30004;
    let t109258 = t109233 + t109235 + t109237 + t109239 + t109241 + t109244 + t109246 + t109248 + t109250 + t109252 + t109254 + t109256 + t109006 + F::cast_from(2.0_f64) * t105850;
    t109258
}
