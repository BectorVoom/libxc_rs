//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta317 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1226;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1227;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta317<F: Float>(t12810: F, t3629: F, t3626: F, t221: F, t462: F, t68: F, t461: F, t1209: F, t3766: F, t5330: F, t1214: F, t3603: F, t3720: F, t1250: F, t12726: F, t11772: F, t3623: F, t3717: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t12846, t12847, t12853, t12854, t12855, t12856) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1226::<F>(t12810, t3629, t3626, t221, t462, t68, t461, t1209, t3766, t5330, t1214, t3603);
        let (t12857, t12858, t12861, t12862, t12865, t12866) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1227::<F>(t12810, t12856, t3720, t1250, t12726, t11772, t3623, t3717);
    (t12846, t12847, t12853, t12854, t12855, t12856, t12857, t12858, t12861, t12862, t12865, t12866)
}
