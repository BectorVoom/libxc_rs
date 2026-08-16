//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1257/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1257(t1134: f64, t1144: f64, t2112: f64, t2119: f64, t2145: f64, t2146: f64, t22060: f64, t22119: f64, t2957: f64, t2964: f64, t2989: f64, t2990: f64, t307: f64, t5990: f64, t6000: f64, t6002: f64, t6054: f64, t7805: f64, t7821: f64, t7824: f64, t786: f64, t7885: f64, t790: f64, t800: f64) -> f64 {
    let t22124 = -0.11853808529283920877e2_f64 * t307 * t6000 * t2989 * t2119 - 0.19756347548806534796e1_f64 * t786 * t7885 - 0.19756347548806534796e1_f64 * t2957 * t2146 - 0.19756347548806534796e1_f64 * t7805 * t800 + 0.13170898365871023197e1_f64 * t307 * t2964 * t6054 - 0.39512695097613069591e1_f64 * t1134 * t6002 - 0.11853808529283920877e2_f64 * t786 * t7821 - 0.19756347548806534796e1_f64 * t2112 * t2990 + 0.39512695097613069591e1_f64 * t307 * t7824 * t2145 - 0.65854491829355115987e0_f64 * t5990 * t1144 - 0.65854491829355115987e0_f64 * t307 * t790 * (t22060 + t22119);
    t22124
}
