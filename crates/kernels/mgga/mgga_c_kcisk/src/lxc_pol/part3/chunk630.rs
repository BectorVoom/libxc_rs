//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 630/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk630<F: Float>(t5355: F, t5524: F, t2038: F, t2041: F, t2040: F, t801: F, t798: F, t2049: F, t5275: F, t5279: F, t5281: F, t5287: F, t5292: F, t5296: F, t5300: F, t5304: F, t5308: F, t5311: F, t5313: F, t5318: F, t5324: F, t5328: F, t5333: F, t5337: F) -> (F, F, F, F, F, F) {
    let t5525 = t5355 + t5524;
    let t5527 = t2038 * t2041;
    let t5531 = F::new(1.0) / t2040 / t801;
    let t5532 = t798 * t5531;
    let t5533 = t2049 * t2049;
    let t5552 = F::new(0.9375e-1) * t5275 - F::new(0.1875e0) * t5279 + F::new(0.125e0) * t5281 + F::new(0.1875e0) * t5287 - F::new(0.125e0) * t5292 - F::new(0.9375e-1) * t5296 - F::new(0.20833333333333333333e-1) * t5300 + F::new(0.625e-1) * t5304 - F::new(0.101171875e-1) * t5308 + F::new(0.20234375e-1) * t5311 - F::new(0.26979166666666666666e-1) * t5313 - F::new(0.20234375e-1) * t5318 + F::new(0.26979166666666666666e-1) * t5324 + F::new(0.101171875e-1) * t5328 - F::new(0.44965277777777777777e-2) * t5333 - F::new(0.13489583333333333333e-1) * t5337;
    (t5525, t5527, t5531, t5532, t5533, t5552)
}
