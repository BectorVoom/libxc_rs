//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 910/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk910<F: Float>(t3361: F, t635: F, t10356: F, t12254: F, t141: F, t1146: F, t2439: F, t3424: F, t698: F, t3421: F, t57: F, t3417: F, t3362: F, t1145: F, t10326: F, t1121: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12256 = 1.0 / t3361 / t635;
    let t12257 = t12256 * t10356;
    let t12258 = t12254 * t12257;
    let t12259 = t141 * t12258;
    let t12261 = t2439 * t1146;
    let t12263 = t698 * t3424;
    let t12265 = t698 * t3421;
    let t12267 = t3361 * t57;
    let t12268 = 1.0 / t12267;
    let t12269 = t12268 * t10356;
    let t12270 = t3417 * t12269;
    let t12271 = t141 * t12270;
    let t12273 = t3362 * t10356;
    let t12274 = t1145 * t12273;
    let t12275 = t141 * t12274;
    let t12277 = t1121 * t10326;
    (t12256, t12257, t12259, t12261, t12263, t12265, t12268, t12269, t12271, t12273, t12275, t12277)
}
