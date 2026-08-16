//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta608 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1917;
use chunk1::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1918;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta608(t22881: f64, t5187: f64, t6637: f64, t6888: f64, t16049: f64, t1992: f64, t81027: f64, t16052: f64, t22897: f64, t26392: f64, t80670: f64, t16419: f64, t6976: f64, t22705: f64, t26422: f64, t81228: f64, t16040: f64, t22633: f64, t3807: f64, t54854: f64, t550: f64, t26331: f64, t26421: f64, t26446: f64, t3719: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t90829, t90832, t90835, t90837, t90840) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1917(t22881, t5187, t6637, t6888, t16049, t1992, t81027, t16052, t22897, t26392, t80670, t16419, t6976);
        let (t90844, t90848, t90852, t90856) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1918(t22705, t26422, t81228, t16040, t22633, t3807, t6976, t1992, t54854, t550, t26331, t26421, t26446, t3719);
    (t90829, t90832, t90835, t90837, t90840, t90844, t90848, t90852, t90856)
}
