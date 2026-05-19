//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 568/1426 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk568<F: Float>(t3182: F, t3184: F, t3190: F, t3194: F, t3199: F, t3202: F, t3204: F, t3207: F, t3210: F, t3213: F, t3262: F, t1066: F, t883: F) -> (F, F) {
    let t3263 = -F::cast_from(0.46971924784082831588e-3_f64) * t3182 + F::cast_from(0.28183154870449698953e-3_f64) * t3184 - F::cast_from(0.28183154870449698953e-3_f64) * t3190 - F::cast_from(0.93943849568165663176e-5_f64) * t3194 + F::cast_from(0.16703216453219854913e-4_f64) * t3199 + F::cast_from(0.28183154870449698953e-3_f64) * t3202 + F::cast_from(0.37186107120732241674e-4_f64) * t3204 - F::cast_from(0.28183154870449698953e-3_f64) * t3207 - F::cast_from(0.1778266270470648716e-4_f64) * t3210 + F::cast_from(0.41036913933938047292e-5_f64) * t3213 + t3262;
    let t3265 = t1066 * t883;
    (t3263, t3265)
}
