//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1025/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1025<F: Float>(t12234: F, t3516: F, t1196: F, t1130: F, t3376: F, t1151: F, t3379: F, t3428: F, t1126: F, t3432: F, t3436: F, t3431: F, t418: F) -> (F, F, F, F, F) {
    let t12235 = t12234 * t3516;
    let t12237 = F::cast_from(0.35089341735807877242e1_f64) * t1196 * t12235;
    let t12238 = t3376 * t1130;
    let t12240 = F::cast_from(3.0_f64) * t12238 * t1151;
    let t12242 = F::cast_from(3.0_f64) * t3379 * t3428;
    let t12243 = t1126 * t3432;
    let t12245 = F::cast_from(0.48245938496077605201e2_f64) * t12243 * t3436;
    let t12247 = F::cast_from(1.0_f64) / t3431 / t418;
    (t12237, t12240, t12242, t12245, t12247)
}
