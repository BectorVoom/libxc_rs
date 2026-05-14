//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 637/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk637<F: Float>(t187: F, t6800: F, t5636: F, t2522: F, t2562: F, t2579: F, t2587: F, t3871: F, t3873: F, t4027: F, t6780: F, t6802: F, t2569: F, t3854: F, t3859: F, t3862: F, t3865: F, t3867: F, t4035: F, t4037: F, t4042: F, t6777: F, t6778: F, t6779: F) -> (F, F, F, F) {
    let t6827 = 0.19751673498613801407e-1 * t6800 * t187;
    let t6828 = 0.36622894612013090108e-3 * t5636;
    let t6829 = t6827 + t3873 - t2522 + t6802 - t4027 + t2579 + t2587 - t6828 + t3871 - t6780 - t2562;
    let t6830 = -t6777 - t6778 - t2569 + t6779 + t3854 - t3867 - t4035 - t4037 + t3859 + t3862 + t3865 + t4042;
    (t6827, t6828, t6829, t6830)
}
