//! MGGA_C_TPSSLOC lxc pol kernel — _part27_v4rho3sigma_3 meta504 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1898;
use chunk1::mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1899;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_meta504(t25249: f64, t776: f64, t25248: f64, t25038: f64, t7510: f64, t814: f64, t829: f64, t7528: f64, t794: f64, t6562: f64, t1509: f64, t1902: f64, t1510: f64, t22992: f64, t13380: f64, t232: f64, t6646: f64, t1888: f64, t1499: f64, t23002: f64, t23014: f64, t23026: f64, t23028: f64, t23032: f64, t23166: f64, t23169: f64, t23174: f64, t25239: f64, t25243: f64, t25246: f64, t2617: f64, t4291: f64, t6660: f64, t7533: f64, t812: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t25250, t25251, t25252, t25255, t25256, t25258, t25259, t25261) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1898(t25249, t776, t25248, t25038, t7510, t814, t829, t7528, t794, t6562, t1509, t1902);
        let (t25262, t25269, t25272, t25273, t25276) = mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1899(t25261, t829, t1510, t22992, t13380, t232, t6646, t1888, t1499, t23002, t23014, t23026, t23028, t23032, t23166, t23169, t23174, t25239, t25243, t25246, t25252, t25256, t25259, t2617, t4291, t6660, t7533, t812);
    (t25250, t25251, t25255, t25256, t25258, t25261, t25262, t25269, t25272, t25273, t25276)
}
