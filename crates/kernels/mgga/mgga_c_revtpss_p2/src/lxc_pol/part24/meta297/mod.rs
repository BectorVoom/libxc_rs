//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta297 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1082;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta297<F: Float>(t1219: F, t6667: F, t247: F, t3634: F, t6429: F, t1261: F, t5378: F, t5391: F, t17394: F, t4890: F, t3767: F, t3782: F) -> (F, F, F, F, F, F, F) {
        let (t20966, t20973, t20974, t21001, t21013, t21014, t21017) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1082::<F>(t1219, t6667, t247, t3634, t6429, t1261, t5378, t5391, t17394, t4890, t3767, t3782);
    (t20966, t20973, t20974, t21001, t21013, t21014, t21017)
}
