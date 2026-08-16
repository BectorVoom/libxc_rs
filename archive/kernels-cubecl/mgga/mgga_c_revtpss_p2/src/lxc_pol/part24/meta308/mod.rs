//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta308 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1093;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1094;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta308<F: Float>(t22022: F, t2661: F, t550: F, t6861: F, t4003: F, t9934: F, t3989: F, t6856: F, t3957: F, t6884: F, t6850: F, t9744: F, t125: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t22023, t22025, t22026, t22027, t22028, t22030, t22038, t22044) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1093::<F>(t22022, t2661, t550, t6861, t4003, t9934, t3989, t6856, t3957, t6884, t6850, t9744);
        let t22046 = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1094::<F>(t125, t6861);
    (t22023, t22025, t22026, t22027, t22028, t22030, t22038, t22044, t22046)
}
