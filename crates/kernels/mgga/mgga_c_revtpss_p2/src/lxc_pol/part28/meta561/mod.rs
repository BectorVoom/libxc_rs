//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta561 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2019;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta561<F: Float>(t7058: F, t93146: F, t2470: F, t25295: F, t2453: F, t25309: F, t25301: F, t25304: F, t7064: F, t251: F, t25410: F, t136: F, t137: F, t1949: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t93147, t93150, t93151, t93158, t93161, t93167, t93169, t93170, t93172) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2019::<F>(t7058, t93146, t2470, t25295, t2453, t25309, t25301, t25304, t7064, t251, t25410, t136, t137, t1949);
    (t93147, t93150, t93151, t93158, t93161, t93167, t93169, t93170, t93172)
}
