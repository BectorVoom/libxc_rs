//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1329/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1329<F: Float>(t2435: F, t26061: F, t1385: F, t7274: F, t1398: F, t4131: F, t543: F, t2453: F, t26053: F, t9676: F, t4078: F, t689: F, t7242: F) -> (F, F, F, F, F) {
    let t94714 = t2435 * t26061;
    let t94716 = t1385 * t7274;
    let t94721 = t4131 * t1398 * t543;
    let t94725 = t2453 * t26053;
    let t94726 = t94725 * t9676;
    let t94729 = t689 * t7242 * t4078;
    (t94714, t94716, t94721, t94726, t94729)
}
