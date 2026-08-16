//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta245 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1090;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1091;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta245<F: Float>(t1250: F, t482: F, t6622: F, t1042: F, t1794: F, t3604: F, t3611: F, t1469: F, t3628: F, t5351: F, t3626: F, t6587: F, t371: F, t372: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t6624, t6625, t6628) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1090::<F>(t1250, t482, t6622, t1042, t1794);
        let (t6630, t6631, t6634, t6635, t6638, t6639, t6640, t6645, t6647) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1091::<F>(t482, t6628, t3604, t1042, t3611, t1469, t3628, t5351, t3626, t6587, t371, t372);
    (t6624, t6625, t6628, t6630, t6631, t6634, t6635, t6638, t6639, t6640, t6645, t6647)
}
