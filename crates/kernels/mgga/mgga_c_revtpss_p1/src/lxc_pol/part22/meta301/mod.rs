//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta301 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1733;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1734;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta301<F: Float>(t10014: F, t4104: F, t268: F, t4056: F, t543: F, t675: F, t4101: F, t555: F, t5744: F, t786: F, t3923: F, t4003: F, t2435: F, t4093: F, t4083: F, t9303: F, t4066: F, t545: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t10015, t10019, t10020, t10022) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1733::<F>(t10014, t4104, t268, t4056, t543, t675, t4101, t555, t5744);
        let (t10023, t10024, t10026, t10027, t10032, t10035, t10039) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk1734::<F>(t10022, t786, t3923, t675, t268, t4003, t2435, t4093, t4083, t9303, t4066, t545);
    (t10015, t10019, t10020, t10022, t10023, t10024, t10026, t10027, t10032, t10035, t10039)
}
