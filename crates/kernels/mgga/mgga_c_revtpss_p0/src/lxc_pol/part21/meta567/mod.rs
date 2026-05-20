//! MGGA_C_REVTPSS lxc pol kernel — _part21_v4rho4_1 meta567 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2265;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_meta567<F: Float>(t5056: F, t5405: F, t3626: F, t12803: F, t471: F, t1715: F, t12810: F, t3603: F, t3362: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t17668, t17669, t17672, t17673, t17674, t17677, t17678, t17679, t17682, t17683, t17684, t17687) = mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2265::<F>(t5056, t5405, t3626, t12803, t471, t1715, t12810, t3603, t3362);
    (t17668, t17669, t17672, t17673, t17674, t17677, t17678, t17679, t17682, t17683, t17684, t17687)
}
