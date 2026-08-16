//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3224/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3224(t1214: f64, t1248: f64, t12717: f64, t12723: f64, t1285: f64, t1287: f64, t13127: f64, t13129: f64, t17854: f64, t1811: f64, t20850: f64, t20900: f64, t21439: f64, t21607: f64, t24989: f64, t3755: f64, t45659: f64, t5284: f64, t5436: f64, t5449: f64, t5474: f64, t5491: f64, t59871: f64, t59872: f64, t6695: f64, t82293: f64, t82886: f64, t82899: f64, t83662: f64, t84462: f64) -> f64 {
    let t84816 = -0.23707617058567841754e2_f64 * t59871 * t82886 * t59872 * t1248 + 0.39512695097613069591e1_f64 * t45659 * t82293 * t17854 * t1214 + 0.19756347548806534796e1_f64 * t21439 * t5474 - 0.19756347548806534796e1_f64 * t20850 * t5449 + 0.19756347548806534796e1_f64 * t1285 * t1811 * t20900 * t1287 + 0.19756347548806534796e1_f64 * t1285 * t6695 * t5284 * t1287 - 0.19756347548806534796e1_f64 * t12723 * t24989 - 0.19756347548806534796e1_f64 * t3755 * t82899 * t1287 + 0.65854491829355115987e0_f64 * t13127 * t84462 * t13129 + 0.39512695097613069591e1_f64 * t5436 * t21607 + 0.19756347548806534796e1_f64 * t21439 * t5491 + 0.39512695097613069591e1_f64 * t12717 * t83662 * t1287;
    t84816
}
