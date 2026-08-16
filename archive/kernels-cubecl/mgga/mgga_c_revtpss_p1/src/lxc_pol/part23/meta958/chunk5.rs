//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3218/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3218<F: Float>(t1234: F, t1269: F, t12702: F, t1280: F, t1285: F, t1287: F, t12987: F, t17289: F, t17821: F, t17958: F, t21436: F, t21452: F, t21465: F, t21480: F, t21541: F, t21558: F, t21579: F, t24616: F, t24770: F, t24974: F, t3670: F, t3759: F, t44843: F, t5216: F, t5245: F, t5436: F, t6573: F, t6723: F, t6741: F, t82514: F, t83551: F) -> F {
    let t84605 = F::cast_from(0.15805078039045227836e2_f64) * t44843 * t1280 * t82514 - F::cast_from(0.19756347548806534796e1_f64) * t17958 * t21480 - F::cast_from(0.19756347548806534796e1_f64) * t1234 * t21541 * t5245 + F::cast_from(0.39512695097613069591e1_f64) * t3670 * t17821 * t6573 + F::cast_from(0.39512695097613069591e1_f64) * t12702 * t24974 + F::cast_from(0.39512695097613069591e1_f64) * t5436 * t21436 + F::cast_from(0.13170898365871023197e1_f64) * t3670 * t1280 * t83551 - F::cast_from(0.39512695097613069591e1_f64) * t12987 * t3759 * t24616 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t1269 * t24770 * t1287 + F::cast_from(0.19756347548806534796e1_f64) * t5216 * t6741 - F::cast_from(0.39512695097613069592e1_f64) * t21579 * t21558 - F::cast_from(0.19756347548806534796e1_f64) * t17289 * t6723 + F::cast_from(0.39512695097613069592e1_f64) * t21452 * t21465;
    t84605
}
