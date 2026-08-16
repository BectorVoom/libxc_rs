//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2242/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2242(t104706: f64, t4890: f64, t104689: f64, t104691: f64, t104752: f64, t104856: f64, t104943: f64, t20771: f64, t20907: f64, t20914: f64, t20938: f64, t20941: f64, t20947: f64, t26880: f64, t29083: f64, t3782: f64, t5270: f64, t5299: f64, t5335: f64, t6635: f64, t7624: f64, t97129: f64, t97174: f64) -> (f64, f64) {
    let t112220 = t104706 * t4890;
    let t112224 = -0.57165357490759649296e-3_f64 * t7624 * t20907 + 0.57165357490759649296e-3_f64 * t104752 * t5299 - 0.42874018118069736972e-3_f64 * t97129 * t6635 + t104689 + t104691 + 0.95275595817932748827e-3_f64 * t104943 * t20947 + 0.57165357490759649296e-3_f64 * t104856 * t20771 - 0.11433071498151929859e-2_f64 * t104943 * t20938 + 0.57165357490759649296e-3_f64 * t97174 * t20941 + 0.57165357490759649296e-3_f64 * t26880 * t20914 + 0.60976381323476959248e-2_f64 * t29083 * t5270 + 0.45732285992607719436e-2_f64 * t3782 * t112220 * t5335;
    (t112220, t112224)
}
