//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3218/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3218(t1234: f64, t1269: f64, t12702: f64, t1280: f64, t1285: f64, t1287: f64, t12987: f64, t17289: f64, t17821: f64, t17958: f64, t21436: f64, t21452: f64, t21465: f64, t21480: f64, t21541: f64, t21558: f64, t21579: f64, t24616: f64, t24770: f64, t24974: f64, t3670: f64, t3759: f64, t44843: f64, t5216: f64, t5245: f64, t5436: f64, t6573: f64, t6723: f64, t6741: f64, t82514: f64, t83551: f64) -> f64 {
    let t84605 = 0.15805078039045227836e2_f64 * t44843 * t1280 * t82514 - 0.19756347548806534796e1_f64 * t17958 * t21480 - 0.19756347548806534796e1_f64 * t1234 * t21541 * t5245 + 0.39512695097613069591e1_f64 * t3670 * t17821 * t6573 + 0.39512695097613069591e1_f64 * t12702 * t24974 + 0.39512695097613069591e1_f64 * t5436 * t21436 + 0.13170898365871023197e1_f64 * t3670 * t1280 * t83551 - 0.39512695097613069591e1_f64 * t12987 * t3759 * t24616 + 0.65854491829355115987e0_f64 * t1285 * t1269 * t24770 * t1287 + 0.19756347548806534796e1_f64 * t5216 * t6741 - 0.39512695097613069592e1_f64 * t21579 * t21558 - 0.19756347548806534796e1_f64 * t17289 * t6723 + 0.39512695097613069592e1_f64 * t21452 * t21465;
    t84605
}
