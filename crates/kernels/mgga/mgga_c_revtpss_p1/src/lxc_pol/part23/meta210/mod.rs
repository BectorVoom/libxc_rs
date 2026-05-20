//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta210 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1239;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1240;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1241;
use chunk3::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1242;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta210<F: Float>(t231: F, t5977: F, t827: F, t828: F, t124: F, t5962: F, t800: F, t5966: F, t2477: F, t190: F, t5825: F, t706: F, t5819: F, t2611: F, t2498: F, t2518: F, t2522: F, t2562: F, t2569: F, t2579: F, t2587: F, t2610: F, t2621: F, t2628: F, t2632: F, t5924: F, t5925: F, t5927: F, t5943: F, t5945: F, t5947: F, t5948: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let t5978 = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1239::<F>(t231, t5977);
        let (t5980, t5984) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1240::<F>(t5978, t827, t828, t124, t5962);
        let (t5985, t5988) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1241::<F>(t5984, t800, t124, t5966);
        let (t5989, t5993, t5999, t6001, t6002, t6004, t6005) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1242::<F>(t5988, t800, t2477, t5966, t828, t190, t5825, t706, t5819, t2611, t2498, t2518, t2522, t2562, t2569, t2579, t2587, t2610, t2621, t2628, t2632, t5924, t5925, t5927, t5943, t5945, t5947, t5948);
    (t5978, t5980, t5984, t5985, t5988, t5989, t5993, t5999, t6001, t6002, t6004, t6005)
}
