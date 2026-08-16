//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta504 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1511;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1512;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta504<F: Float>(t2661: F, t2662: F, t4352: F, t6017: F, t23285: F, t2741: F, t23289: F, t6035: F, t61625: F, t23342: F, t2652: F, t221: F, t23114: F, t2674: F, t40683: F, t14648: F, t14832: F, t5962: F, t23346: F, t231: F, t76569: F, t23244: F, t243: F, t10871: F, t40693: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t76764, t76767, t76793, t76797, t76804, t76808) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1511::<F>(t2661, t2662, t4352, t6017, t23285, t2741, t23289, t6035, t61625, t23342, t2652, t221, t23114, t2674, t40683);
        let (t76812, t76814, t76818, t76823, t76827) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1512::<F>(t14648, t14832, t2661, t5962, t23346, t2652, t231, t2662, t76569, t23244, t243, t10871, t40693);
    (t76764, t76767, t76793, t76797, t76804, t76808, t76812, t76814, t76818, t76823, t76827)
}
