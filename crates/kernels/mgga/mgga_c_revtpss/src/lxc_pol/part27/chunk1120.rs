//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1120/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1120<F: Float>(t1398: F, t4131: F, t543: F, t2453: F, t26053: F, t9676: F, t4078: F, t689: F, t7242: F, t1358: F, t2439: F, t7274: F, t785: F, t26064: F, t3920: F, t1444: F, t4004: F) -> (F, F, F, F, F, F) {
    let t94721 = t4131 * t1398 * t543;
    let t94725 = t2453 * t26053;
    let t94726 = t94725 * t9676;
    let t94729 = t689 * t7242 * t4078;
    let t94733 = t2439 * t785 * t7274 * t1358;
    let t94735 = t26064 * t3920;
    let t94737 = t4004 * t1444;
    (t94721, t94726, t94729, t94733, t94735, t94737)
}
