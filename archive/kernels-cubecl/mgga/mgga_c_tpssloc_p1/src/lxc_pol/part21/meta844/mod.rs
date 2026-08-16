//! MGGA_C_TPSSLOC lxc pol kernel — _part21_v4rho4_2 meta844 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;
mod chunk3;
mod chunk4;
mod chunk5;
mod chunk6;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3049;
use chunk1::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3050;
use chunk2::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3051;
use chunk3::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3052;
use chunk4::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3053;
use chunk5::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3054;
use chunk6::mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3055;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_meta844<F: Float>(t12606: F, t4723: F, t123: F, t3240: F, t18226: F, t690: F, t18222: F, t2250: F, t5971: F, t1088: F, t18210: F, t2244: F, t4728: F, t11147: F, t5398: F, t11145: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t63394, t63396) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3049::<F>(t12606, t4723, t123, t3240);
        let t63398 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3050::<F>(t18226, t690);
        let t63400 = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3051::<F>(t18222, t690);
        let (t63402, t63404) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3052::<F>(t2250, t5971, t1088, t123);
        let (t63406, t63408) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3053::<F>(t18210, t2244, t1088, t123);
        let (t63410, t63412) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3054::<F>(t12606, t4728, t1088, t123);
        let (t63415, t63417) = mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3055::<F>(t11147, t2244, t5398, t11145, t123);
    (t63394, t63396, t63398, t63400, t63402, t63404, t63406, t63408, t63410, t63412, t63415, t63417)
}
