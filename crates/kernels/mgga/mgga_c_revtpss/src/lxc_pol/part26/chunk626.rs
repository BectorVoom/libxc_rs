//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 626/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk626<F: Float>(t1399: F, t221: F, t4019: F, t4018: F, t1317: F, t1331: F, t1333: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t3852: F, t3854: F, t3871: F, t3873: F) -> (F, F, F, F, F) {
    let t4021 = t4019 * t221 * t1399;
    let t4022 = t4018 * t4021;
    let t4024 = t1317 * t1331;
    let t4025 = F::new(8.0) * t4024;
    let t4027 = F::new(8.0) * t1317 * t1333;
    let t4028 = t3873 - t2522 + t4025 + t4027 + t2579 + t2587 + t3871 + t3852 - t2562 - t2569 + t3854;
    (t4021, t4022, t4025, t4027, t4028)
}
