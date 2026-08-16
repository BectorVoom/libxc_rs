//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1264/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1264(t11054: f64, t1134: f64, t1144: f64, t12389: f64, t1306: f64, t135: f64, t158: f64, t26809: f64, t273: f64, t2957: f64, t2965: f64, t29753: f64, t2990: f64, t30193: f64, t30195: f64, t30197: f64, t30200: f64, t30203: f64, t30205: f64, t30208: f64, t30211: f64, t30807: f64, t30982: f64, t311: f64, t3670: f64, t3676: f64, t3695: f64, t800: f64, t805: f64, t9634: f64, t9648: f64, t9651: f64, t9657: f64) -> f64 {
    let t30990 = -t29753 + t135 * t273 * (0.65854491829355115987e0_f64 * t30807 * t158 * t311 - 0.65854491829355115987e0_f64 * t11054 * t800 - 0.19756347548806534796e1_f64 * t9634 * t1144 + 0.39512695097613069592e1_f64 * t3670 * t2965 - 0.19756347548806534796e1_f64 * t3670 * t2990 + 0.39512695097613069591e1_f64 * t2957 * t3676 - 0.11853808529283920877e2_f64 * t1134 * t9648 + 0.79025390195226139182e1_f64 * t1134 * t9651 - 0.19756347548806534796e1_f64 * t2957 * t3695 + 0.39512695097613069592e1_f64 * t1134 * t9657 + t30982) * t805 - t30193 + t30195 - t30197 + 6.0_f64 * t1306 * t26809 * t12389 + t30200 + t30203 - t30205 - t30208 - t30211;
    t30990
}
