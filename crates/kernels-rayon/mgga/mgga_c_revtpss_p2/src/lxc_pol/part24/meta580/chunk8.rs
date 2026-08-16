//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1801/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1801(t1811: f64, t24543: f64, t1234: f64, t12717: f64, t1280: f64, t1287: f64, t12987: f64, t13127: f64, t13129: f64, t13148: f64, t13149: f64, t21541: f64, t24616: f64, t24951: f64, t24981: f64, t44843: f64, t460: f64, t489: f64, t5326: f64, t5486: f64, t59817: f64, t60019: f64, t6564: f64, t6587: f64, t6717: f64, t6741: f64, t72386: f64, t90059: f64, t91037: f64, t91403: f64) -> (f64, f64) {
    let t91610 = t1811 * t24543;
    let t91642 = 0.15805078039045227836e2_f64 * t13148 * t91610 * t13149 - 0.15805078039045227836e2_f64 * t12987 * t5486 * t24616 + 0.15805078039045227836e2_f64 * t44843 * t1280 * t91037 + 0.26341796731742046395e1_f64 * t13127 * t91610 * t13129 - 0.15805078039045227836e2_f64 * t72386 * t6717 + 0.15805078039045227836e2_f64 * t60019 * t24981 + 0.15805078039045227836e2_f64 * t59817 * t24981 + 0.79025390195226139183e1_f64 * t12717 * t90059 * t1287 + 0.39512695097613069592e1_f64 * t6564 * t6741 - 0.39512695097613069592e1_f64 * t1234 * t21541 * t6587 - 0.79025390195226139183e1_f64 * t5326 * t24951 + 0.65854491829355115987e0_f64 * t460 * t489 * t91403;
    (t91610, t91642)
}
