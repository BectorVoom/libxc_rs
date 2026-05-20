//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta251 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1017;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta251<F: Float>(t15670: F, t366: F, t245: F, t4890: F, t3088: F, t3317: F, t372: F, t4823: F, t1087: F, t11773: F, t4801: F, t1062: F, t4857: F) -> (F, F, F, F, F, F, F, F) {
        let (t15671, t15687, t15688, t15689, t15696, t15700, t15701, t15707) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1017::<F>(t15670, t366, t245, t4890, t3088, t3317, t372, t4823, t1087, t11773, t4801, t1062, t4857);
    (t15671, t15687, t15688, t15689, t15696, t15700, t15701, t15707)
}
