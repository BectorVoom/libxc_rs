//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1071/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1071<F: Float>(t11882: F, t11885: F, t11903: F, t11906: F, t11908: F, t12235: F, t12236: F, t12237: F, t12238: F, t12239: F, t12240: F, t12243: F, t12244: F, t12245: F, t12246: F, t12247: F, t12251: F, t12252: F, t12253: F) -> F {
    let t12644 = -t12235 - t12236 + t12237 + t12238 + t12239 + t12240 - F::cast_from(0.90579542097823505428e-7_f64) * t11882 - F::cast_from(0.52838066223730378166e-7_f64) * t11885 + t12243 - t12244 - t12245 - t12246 - t12247 + F::cast_from(0.90579542097823505428e-7_f64) * t11903 - F::cast_from(0.18115908419564701086e-6_f64) * t11906 + F::cast_from(0.18115908419564701086e-6_f64) * t11908 - t12251 - t12252 + t12253;
    t12644
}
