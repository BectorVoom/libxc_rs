//! MGGA_C_REVTPSS lxc pol kernel — _part29_v4rho3sigma_4 meta587 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1940;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_meta587<F: Float>(t1493: F, t2248: F, t77: F, t2315: F, t2259: F, t4173: F, t38: F, t60248: F, t2251: F, t28104: F, t644: F, t13517: F, t196: F, t197: F) -> (F, F, F, F, F, F, F) {
        let (t101337, t101350, t101357, t101360, t101376, t101399, t101435) = mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1940::<F>(t1493, t2248, t77, t2315, t2259, t4173, t38, t60248, t2251, t28104, t644, t13517, t196, t197);
    (t101337, t101350, t101357, t101360, t101376, t101399, t101435)
}
