//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3219/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3219(t13045: f64, t6622: f64, t1204: f64, t1248: f64, t13148: f64, t13149: f64, t17192: f64, t17846: f64, t20900: f64, t21456: f64, t21459: f64, t21468: f64, t21473: f64, t24915: f64, t24981: f64, t43350: f64, t45707: f64, t45852: f64, t471: f64, t5332: f64, t5446: f64, t5463: f64, t5464: f64, t59650: f64, t59657: f64, t59681: f64, t59737: f64, t6717: f64, t72270: f64, t72386: f64, t82886: f64, t84462: f64) -> f64 {
    let t84636 = t13045 * t6622;
    let t84641 = -0.19756347548806534796e1_f64 * t21456 * t21468 - 0.19756347548806534796e1_f64 * t72270 * t5446 - 0.39512695097613069591e1_f64 * t59657 * t6717 + 0.65854491829355115987e0_f64 * t1204 * t24915 + 0.39512695097613069591e1_f64 * t13148 * t84462 * t13149 - 0.65854491829355115987e0_f64 * t59737 * t82886 * t43350 * t1248 * t471 - 0.19756347548806534796e1_f64 * t17192 * t21459 - 0.39512695097613069591e1_f64 * t72386 * t5446 + 0.39512695097613069591e1_f64 * t45707 * t24981 + 0.39512695097613069591e1_f64 * t45852 * t24981 + 0.19756347548806534796e1_f64 * t59681 * t21473 + 0.39512695097613069591e1_f64 * t5463 * t5332 * t5464 * t20900 + 0.11853808529283920877e2_f64 * t17846 * t59650 * t84636 * t1248;
    t84641
}
