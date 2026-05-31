//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1355/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1355<F: Float>(t2778: F, t39515: F, t39501: F, t871: F, t10115: F, t225: F, t880: F, t10866: F, t232: F, t235: F, t239: F, t820: F) -> (F, F, F, F, F, F) {
    let t40314 = F::cast_from(0.11564373972601816912e-1_f64) * t39515 * t2778;
    let t40316 = F::cast_from(0.56911289235245161963e-1_f64) * t39501 * t871;
    let t40317 = t10115 * t225;
    let t40318 = t40317 * t880;
    let t40321 = F::cast_from(1.0_f64) / t10866 / t232;
    let t40322 = t40321 * t235;
    let t40324 = t820 * t40322 * t239;
    (t40314, t40316, t40317, t40318, t40321, t40324)
}
