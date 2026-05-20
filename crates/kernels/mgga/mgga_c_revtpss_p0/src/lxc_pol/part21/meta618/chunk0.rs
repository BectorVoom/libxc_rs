//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2372/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2372<F: Float>(t10115: F, t225: F, t880: F, t10866: F, t232: F, t235: F, t2723: F, t10666: F, t221: F, t2484: F, t2485: F, t2482: F, t2719: F, t596: F) -> (F, F, F, F, F, F, F) {
    let t40317 = t10115 * t225;
    let t40318 = t40317 * t880;
    let t40321 = F::new(1.0) / t10866 / t232;
    let t40322 = t40321 * t235;
    let t40325 = t2723 * t2723;
    let t40333 = t2484 * t2485 * t221 * t10666;
    let t40336 = t2482 * t2719 * t596;
    (t40317, t40318, t40321, t40322, t40325, t40333, t40336)
}
