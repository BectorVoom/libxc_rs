//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta613 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2044;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2045;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta613<F: Float>(t27873: F, t94886: F, t27845: F, t689: F, t25904: F, t25899: F, t94649: F, t97685: F, t25898: F, t7925: F, t94849: F, t1032: F, t5710: F, t1426: F, t7063: F, t7286: F, t27852: F, t25950: F, t27888: F, t25953: F, t27884: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t97945, t97949, t97951, t97953, t97956, t97960) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2044::<F>(t27873, t94886, t27845, t689, t25904, t25899, t94649, t97685, t25898, t7925, t94849, t1032, t5710);
        let (t97961, t97964, t97968, t97974, t97976, t97985) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2045::<F>(t1426, t97960, t7063, t7286, t27852, t689, t25904, t25899, t25950, t27888, t25953, t27884);
    (t97945, t97949, t97951, t97953, t97956, t97960, t97961, t97964, t97968, t97974, t97976, t97985)
}
