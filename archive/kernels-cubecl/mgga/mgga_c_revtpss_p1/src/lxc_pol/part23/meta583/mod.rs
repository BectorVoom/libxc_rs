//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta583 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2208;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2209;
use chunk2::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2210;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta583<F: Float>(t23485: F, t904: F, t128: F, t23474: F, t23481: F, t2908: F, t141: F, t930: F, t4573: F, t5825: F, t2850: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
        let (t23486, t23487) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2208::<F>(t23485, t904, t128);
        let (t23489, t23490) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2209::<F>(t23474, t904, t128);
        let (t23492, t23493, t23495, t23496, t23499, t23500, t23501) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2210::<F>(t23481, t2908, t141, t23485, t930, t4573, t5825, t2850, t128);
    (t23486, t23487, t23489, t23490, t23492, t23493, t23495, t23496, t23499, t23500, t23501)
}
