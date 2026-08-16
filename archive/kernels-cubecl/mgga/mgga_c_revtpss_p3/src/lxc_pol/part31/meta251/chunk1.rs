//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1107/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1107<F: Float>(t2569: F, t3854: F, t3859: F, t3862: F, t3865: F, t3867: F, t4035: F, t4037: F, t4042: F, t6777: F, t6778: F, t6779: F) -> F {
    let t6830 = -t6777 - t6778 - t2569 + t6779 + t3854 - t3867 - t4035 - t4037 + t3859 + t3862 + t3865 + t4042;
    t6830
}
