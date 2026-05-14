//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 966/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk966<F: Float>(t38225: F, t10968: F, t6262: F, t6855: F, t10930: F, t158: F, t2304: F, t2317: F, t3434: F, t357: F, t6854: F, t862: F, t1615: F, t875: F, t269: F, t3438: F) -> (F, F, F, F, F) {
    let t38226 = 0.91462949374725084942e-3 * t38225;
    let t38228 = t6855 * t6262 * t10968;
    let t38229 = 0.64980365807044550255e-5 * t38228;
    let t38233 = t3434 * t2304 * t2317 * t158 * t10930;
    let t38234 = 0.5854811038705731867e-3 * t38233;
    let t38240 = t862 * t357 * t6854;
    let t38241 = t1615 * t875;
    let t38244 = t38240 * t38241 * t3438 * t269;
    (t38226, t38229, t38234, t38241, t38244)
}
