//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1169/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1169<F: Float>(t13934: F, t2549: F, t2562: F, t38974: F, t883: F, t943: F, t43312: F, t43315: F, t43318: F, t43321: F, t43325: F, t43326: F, t43330: F, t43335: F, t43339: F) -> F {
    let t47768 = t2549 * t13934;
    let t47772 = t943 * t2562 * t883 * t38974;
    let t47777 = -F::cast_from(0.32043859292259267849e-3_f64) * t47768 - F::cast_from(0.32043859292259267849e-3_f64) * t47772 + t43312 + t43315 + F::cast_from(0.15381052460284448567e-1_f64) * t43318 + t43321 + t43325 - F::cast_from(0.32043859292259267849e-3_f64) * t43326 - F::cast_from(0.32043859292259267849e-3_f64) * t43330 + t43335 - t43339;
    t47777
}
