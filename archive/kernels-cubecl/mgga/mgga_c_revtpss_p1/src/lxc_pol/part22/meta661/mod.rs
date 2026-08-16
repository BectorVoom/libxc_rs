//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta661 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2618;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta661<F: Float>(t1248: F, t13045: F, t20956: F, t3720: F, t5341: F, t1219: F, t6667: F, t247: F, t3634: F, t6429: F, t1261: F, t12856: F, t20795: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t20957, t20958, t20959, t20962, t20963, t20966, t20973, t20974, t20977) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2618::<F>(t1248, t13045, t20956, t3720, t5341, t1219, t6667, t247, t3634, t6429, t1261, t12856, t20795);
    (t20957, t20958, t20959, t20962, t20963, t20966, t20973, t20974, t20977)
}
