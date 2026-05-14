//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 650/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk650<F: Float>(t2993: F, t7705: F, t419: F, t173: F, t1736: F, t2984: F, t11034: F, t3088: F, t420: F, t8119: F, t11008: F, t2248: F, t11013: F, t10998: F, t1527: F, t422: F) -> (F, F, F, F, F, F, F, F, F) {
    let t11259 = t7705 * t2993;
    let t11260 = t419 * t11259;
    let t11262 = t173 * t1736;
    let t11263 = t11262 * t2984;
    let t11264 = t419 * t11263;
    let t11265 = 0.56749874115226337448e-2 * t11264;
    let t11266 = t3088 * t11034;
    let t11267 = t419 * t11266;
    let t11269 = t420 * t8119;
    let t11270 = t11269 * t11008;
    let t11271 = t419 * t11270;
    let t11273 = t2248 * t1736;
    let t11274 = t11273 * t11013;
    let t11275 = t419 * t11274;
    let t11277 = t1527 * t10998;
    let t11278 = t419 * t11277;
    let t11280 = t2248 * t422;
    (t11260, t11262, t11264, t11265, t11267, t11271, t11275, t11278, t11280)
}
