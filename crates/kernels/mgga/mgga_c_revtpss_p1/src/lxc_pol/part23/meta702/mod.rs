//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta702 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2452;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta702<F: Float>(t9860: F, t9866: F, t9863: F, t3869: F, t39532: F, t9575: F, t39538: F, t39427: F, t39535: F, t4038: F, t9372: F, t1317: F, t9428: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t47125, t47127, t47131, t47135, t47138, t47140, t47142, t47147, t47149) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2452::<F>(t9860, t9866, t9863, t3869, t39532, t9575, t39538, t39427, t39535, t4038, t9372, t1317, t9428);
    (t47125, t47127, t47131, t47135, t47138, t47140, t47142, t47147, t47149)
}
