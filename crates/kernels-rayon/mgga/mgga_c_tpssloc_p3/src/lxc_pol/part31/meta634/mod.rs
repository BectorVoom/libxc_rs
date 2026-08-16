//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta634 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1897;
use chunk1::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1898;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta634(t22779: f64, t28060: f64, t19661: f64, t1992: f64, t22897: f64, t19736: f64, t22892: f64, t22893: f64, t28138: f64, t28116: f64, t81228: f64, t81326: f64, t6897: f64, t7700: f64, t90544: f64, t214: f64, t6434: f64, t1985: f64, t6907: f64, t22633: f64, t26215: f64, t90566: f64, t22635: f64, t26354: f64, t5353: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t97463, t97488, t97491, t97494, t97503) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1897(t22779, t28060, t19661, t1992, t22897, t19736, t22892, t22893, t28138, t28116, t81228, t81326);
        let (t97509, t97511, t97513, t97516, t97524) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1898(t6897, t7700, t90544, t214, t6434, t1985, t6907, t22633, t26215, t90566, t1992, t22635, t26354, t5353);
    (t97463, t97488, t97491, t97494, t97503, t97509, t97511, t97513, t97516, t97524)
}
