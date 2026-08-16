//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 569/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk569(t3182: f64, t3184: f64, t3190: f64, t3194: f64, t3199: f64, t3202: f64, t3204: f64, t3207: f64, t3210: f64, t3213: f64, t3262: f64, t1066: f64, t883: f64) -> (f64, f64) {
    let t3263 = -0.46971924784082831588e-3_f64 * t3182 + 0.28183154870449698953e-3_f64 * t3184 - 0.28183154870449698953e-3_f64 * t3190 - 0.93943849568165663176e-5_f64 * t3194 + 0.16703216453219854913e-4_f64 * t3199 + 0.28183154870449698953e-3_f64 * t3202 + 0.37186107120732241674e-4_f64 * t3204 - 0.28183154870449698953e-3_f64 * t3207 - 0.1778266270470648716e-4_f64 * t3210 + 0.41036913933938047292e-5_f64 * t3213 + t3262;
    let t3265 = t1066 * t883;
    (t3263, t3265)
}
