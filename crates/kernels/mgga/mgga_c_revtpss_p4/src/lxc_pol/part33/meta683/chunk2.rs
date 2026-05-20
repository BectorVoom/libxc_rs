//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2242/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2242<F: Float>(t104706: F, t4890: F, t104689: F, t104691: F, t104752: F, t104856: F, t104943: F, t20771: F, t20907: F, t20914: F, t20938: F, t20941: F, t20947: F, t26880: F, t29083: F, t3782: F, t5270: F, t5299: F, t5335: F, t6635: F, t7624: F, t97129: F, t97174: F) -> (F, F) {
    let t112220 = t104706 * t4890;
    let t112224 = -F::cast_from(0.57165357490759649296e-3_f64) * t7624 * t20907 + F::cast_from(0.57165357490759649296e-3_f64) * t104752 * t5299 - F::cast_from(0.42874018118069736972e-3_f64) * t97129 * t6635 + t104689 + t104691 + F::cast_from(0.95275595817932748827e-3_f64) * t104943 * t20947 + F::cast_from(0.57165357490759649296e-3_f64) * t104856 * t20771 - F::cast_from(0.11433071498151929859e-2_f64) * t104943 * t20938 + F::cast_from(0.57165357490759649296e-3_f64) * t97174 * t20941 + F::cast_from(0.57165357490759649296e-3_f64) * t26880 * t20914 + F::cast_from(0.60976381323476959248e-2_f64) * t29083 * t5270 + F::cast_from(0.45732285992607719436e-2_f64) * t3782 * t112220 * t5335;
    (t112220, t112224)
}
