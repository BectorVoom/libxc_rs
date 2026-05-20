//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1338/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1338<F: Float>(t39501: F, t871: F, t10115: F, t225: F, t10866: F, t232: F, t235: F, t239: F, t820: F, t2723: F, t2482: F, t2719: F, t596: F) -> (F, F, F, F, F, F) {
    let t40316 = F::cast_from(0.56911289235245161963e-1_f64) * t39501 * t871;
    let t40317 = t10115 * t225;
    let t40321 = F::new(1.0) / t10866 / t232;
    let t40322 = t40321 * t235;
    let t40324 = t820 * t40322 * t239;
    let t40325 = t2723 * t2723;
    let t40336 = t2482 * t2719 * t596;
    (t40316, t40317, t40321, t40324, t40325, t40336)
}
