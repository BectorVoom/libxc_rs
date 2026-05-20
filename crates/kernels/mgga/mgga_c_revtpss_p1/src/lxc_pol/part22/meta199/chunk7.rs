//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 1264/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1264<F: Float>(t1058: F, t1660: F, t1053: F, t1659: F, t225: F, t4743: F, t366: F, t1065: F, t2857: F) -> (F, F, F, F, F) {
    let t4792 = t1660 * t1058;
    let t4794 = t1659 * t1053;
    let t4797 = t4743 * t225;
    let t4798 = t4797 * t366;
    let t4801 = t1065 * t2857;
    (t4792, t4794, t4797, t4798, t4801)
}
