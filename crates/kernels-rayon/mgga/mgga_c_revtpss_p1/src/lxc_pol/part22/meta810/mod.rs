//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta810 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2912;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2913;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta810(t245: f64, t47247: f64, t2713: f64, t3964: f64, t9714: f64, t3951: f64, t9732: f64, t136: f64, t4010: f64, t220: f64, t9905: f64, t9976: f64, t3926: f64, t9909: f64, t9775: f64, t9981: f64, t1389: f64, t40604: f64, t3961: f64, t9741: f64, t10111: f64, t22: f64, t4092: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t47248, t47259, t47262, t47273, t47274, t47298) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2912(t245, t47247, t2713, t3964, t9714, t3951, t9732, t136, t4010, t220, t9905, t9976);
        let (t47304, t47320, t47337, t47338, t47348) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2913(t3926, t9909, t9775, t9981, t1389, t3964, t40604, t3961, t9741, t10111, t22, t4092);
    (t47248, t47259, t47262, t47273, t47274, t47298, t47304, t47320, t47337, t47338, t47348)
}
