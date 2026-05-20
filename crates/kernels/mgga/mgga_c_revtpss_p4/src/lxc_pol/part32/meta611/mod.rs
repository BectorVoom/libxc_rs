//! MGGA_C_REVTPSS lxc pol kernel — _part32_v4rho3sigma_7 meta611 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1950;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_meta611<F: Float>(t1113: F, t5962: F, t18392: F, t33: F, t100981: F, t106565: F, t6079: F, t6416: F, t775: F, t106501: F, t27799: F, t25759: F, t77441: F) -> (F, F, F, F, F, F, F) {
        let (t107939, t107943, t107947, t107958, t107970, t107974, t107985) = mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1950::<F>(t1113, t5962, t18392, t33, t100981, t106565, t6079, t6416, t775, t106501, t27799, t25759, t77441);
    (t107939, t107943, t107947, t107958, t107970, t107974, t107985)
}
