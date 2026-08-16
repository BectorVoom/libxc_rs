//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta572 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2423;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta572<F: Float>(t18413: F, t2723: F, t10726: F, t2661: F, t231: F, t2662: F, t10703: F, t221: F, t5966: F, t2674: F, t125: F, t5977: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t18414, t18415, t18416, t18418, t18419, t18420, t18423, t18424, t18426) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2423::<F>(t18413, t2723, t10726, t2661, t231, t2662, t10703, t221, t5966, t2674, t125, t5977);
    (t18414, t18415, t18416, t18418, t18419, t18420, t18423, t18424, t18426)
}
