//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 742/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk742<F: Float>(t2230: F, t7990: F, t2226: F, t7987: F, t2147: F, t2229: F, t463: F, t2138: F, t2132: F, t2225: F, t322: F, t7896: F) -> (F, F, F, F, F, F) {
    let t8076 = t7990 * t2230;
    let t8078 = t7987 * t2226;
    let t8081 = t2147 * t2229 * t463;
    let t8082 = t2138 * t8081;
    let t8085 = t2132 * t2225 * t322;
    let t8087 = F::cast_from(0.34694512752820797848e1_f64) * t7896 * t8085;
    (t8076, t8078, t8081, t8082, t8085, t8087)
}
