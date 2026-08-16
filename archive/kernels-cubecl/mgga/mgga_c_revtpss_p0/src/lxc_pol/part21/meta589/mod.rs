//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta589 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2306;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta589<F: Float>(t19979: F, t372: F, t1651: F, t2857: F, t2852: F, t1774: F, t3362: F, t1794: F, t3617: F, t17394: F, t4890: F, t3767: F) -> (F, F, F, F, F, F, F) {
        let (t19980, t20094, t20099, t20921, t20945, t21013, t21014) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2306::<F>(t19979, t372, t1651, t2857, t2852, t1774, t3362, t1794, t3617, t17394, t4890, t3767);
    (t19980, t20094, t20099, t20921, t20945, t21013, t21014)
}
