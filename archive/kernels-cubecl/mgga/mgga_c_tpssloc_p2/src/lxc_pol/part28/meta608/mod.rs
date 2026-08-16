//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta608 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1917;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1918;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta608<F: Float>(t22881: F, t5187: F, t6637: F, t6888: F, t16049: F, t1992: F, t81027: F, t16052: F, t22897: F, t26392: F, t80670: F, t16419: F, t6976: F, t22705: F, t26422: F, t81228: F, t16040: F, t22633: F, t3807: F, t54854: F, t550: F, t26331: F, t26421: F, t26446: F, t3719: F) -> (F, F, F, F, F, F, F, F, F) {
        let (t90829, t90832, t90835, t90837, t90840) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1917::<F>(t22881, t5187, t6637, t6888, t16049, t1992, t81027, t16052, t22897, t26392, t80670, t16419, t6976);
        let (t90844, t90848, t90852, t90856) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1918::<F>(t22705, t26422, t81228, t16040, t22633, t3807, t6976, t1992, t54854, t550, t26331, t26421, t26446, t3719);
    (t90829, t90832, t90835, t90837, t90840, t90844, t90848, t90852, t90856)
}
