//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 129/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk129(t101: f64, t125: f64, t143: f64, t145: f64, t148: f64, t153: f64, t156: f64, t163: f64, t164: f64, t168: f64, t169: f64, t171: f64, t234: f64, t242: f64, t245: f64, t270: f64, t274: f64, t279: f64, t281: f64, t286: f64, t289: f64, t296: f64, t299: f64, t301: f64) -> f64 {
    let t305 = t101 * t143 + (-0.031505407223141116_f64 * t148 * t164 - 0.005388405304614574_f64 * t169 * t171 * t234 * t163) * t125 + (-0.0837628205355044_f64 * t148 * t242 - 0.011938374665504766_f64 * t168 * t245 * t270 + 0.42708890021612717_f64 * t153 * t156 * t274) * t279 - 0.01197423401025461_f64 * t281 * t286 + (-0.031835665774679375_f64 * t169 * t289 * t242 + 0.05332506774217938_f64 * t145 * t274) * t296 + 0.020267214298646783_f64 * t169 * t299 * t274 * t301;
    t305
}
