//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta292 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1075;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta292<F: Float>(t1179: F, t6513: F, t1160: F, t6481: F, t3479: F, t6502: F, t12472: F, t6486: F, t1130: F, t6433: F, t3435: F, t6470: F) -> (F, F, F, F, F, F) {
        let (t20526, t20542, t20618, t20625, t20629, t20644) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1075::<F>(t1179, t6513, t1160, t6481, t3479, t6502, t12472, t6486, t1130, t6433, t3435, t6470);
    (t20526, t20542, t20618, t20625, t20629, t20644)
}
