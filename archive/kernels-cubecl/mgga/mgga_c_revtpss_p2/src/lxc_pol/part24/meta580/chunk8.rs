//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1801/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1801<F: Float>(t1811: F, t24543: F, t1234: F, t12717: F, t1280: F, t1287: F, t12987: F, t13127: F, t13129: F, t13148: F, t13149: F, t21541: F, t24616: F, t24951: F, t24981: F, t44843: F, t460: F, t489: F, t5326: F, t5486: F, t59817: F, t60019: F, t6564: F, t6587: F, t6717: F, t6741: F, t72386: F, t90059: F, t91037: F, t91403: F) -> (F, F) {
    let t91610 = t1811 * t24543;
    let t91642 = F::cast_from(0.15805078039045227836e2_f64) * t13148 * t91610 * t13149 - F::cast_from(0.15805078039045227836e2_f64) * t12987 * t5486 * t24616 + F::cast_from(0.15805078039045227836e2_f64) * t44843 * t1280 * t91037 + F::cast_from(0.26341796731742046395e1_f64) * t13127 * t91610 * t13129 - F::cast_from(0.15805078039045227836e2_f64) * t72386 * t6717 + F::cast_from(0.15805078039045227836e2_f64) * t60019 * t24981 + F::cast_from(0.15805078039045227836e2_f64) * t59817 * t24981 + F::cast_from(0.79025390195226139183e1_f64) * t12717 * t90059 * t1287 + F::cast_from(0.39512695097613069592e1_f64) * t6564 * t6741 - F::cast_from(0.39512695097613069592e1_f64) * t1234 * t21541 * t6587 - F::cast_from(0.79025390195226139183e1_f64) * t5326 * t24951 + F::cast_from(0.65854491829355115987e0_f64) * t460 * t489 * t91403;
    (t91610, t91642)
}
