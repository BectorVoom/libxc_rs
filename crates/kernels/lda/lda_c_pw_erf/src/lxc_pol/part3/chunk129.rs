//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 129/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk129<F: Float>(t101: F, t125: F, t143: F, t145: F, t148: F, t153: F, t156: F, t163: F, t164: F, t168: F, t169: F, t171: F, t234: F, t242: F, t245: F, t270: F, t274: F, t279: F, t281: F, t286: F, t289: F, t296: F, t299: F, t301: F) -> (F,) {
    let t305 = t101 * t143 + (-0.031505407223141116 * t148 * t164 - 0.005388405304614574 * t169 * t171 * t234 * t163) * t125 + (-0.0837628205355044 * t148 * t242 - 0.011938374665504766 * t168 * t245 * t270 + 0.42708890021612717 * t153 * t156 * t274) * t279 - 0.01197423401025461 * t281 * t286 + (-0.031835665774679375 * t169 * t289 * t242 + 0.05332506774217938 * t145 * t274) * t296 + 0.020267214298646783 * t169 * t299 * t274 * t301;
    (t305,)
}
