//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta193 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk922;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta193<F: Float>(t2735: F, t4086: F, t521: F, t9342: F, t14: F, t588: F, t2516: F, t676: F, t3869: F, t2496: F, t4010: F, t73: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t9845, t9854, t9855, t9857, t9863, t9865, t9866, t9868, t9880) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk922::<F>(t2735, t4086, t521, t9342, t14, t588, t2516, t676, t3869, t2496, t4010, t73);
    (t9845, t9854, t9855, t9857, t9863, t9865, t9866, t9868, t9880)
}
