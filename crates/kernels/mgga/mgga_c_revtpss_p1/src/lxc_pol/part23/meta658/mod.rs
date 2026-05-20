//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta658 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2388;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2389;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta658<F: Float>(t11015: F, t2461: F, t2769: F, t786: F, t861: F, t11007: F, t252: F, t11006: F, t256: F, t225: F, t2441: F, t39515: F, t10504: F, t138: F, t886: F, t9302: F, t123: F, t2465: F, t9291: F, t10982: F, t860: F, t9646: F, t10115: F, t251: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t41060, t41066, t41070, t41078, t41095) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2388::<F>(t11015, t2461, t2769, t786, t861, t11007, t252, t11006, t256, t225, t2441, t39515);
        let (t41098, t41102, t41105, t41117) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2389::<F>(t10504, t138, t886, t9302, t123, t2465, t9291, t10982, t860, t9646, t10115, t251);
    (t41060, t41066, t41070, t41078, t41095, t41098, t41102, t41105, t41117)
}
