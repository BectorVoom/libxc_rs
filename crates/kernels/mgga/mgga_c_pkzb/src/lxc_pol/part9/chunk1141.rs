//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1141/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1141<F: Float>(t19775: F, t1008: F, t5075: F, t83: F, t1673: F, t568: F, t1535: F, t16626: F, t17258: F, t19755: F, t19757: F, t19759: F, t19766: F, t19770: F, t2536: F, t2537: F, t2575: F, t5191: F, t7197: F) -> (F, F, F) {
    let t19776 = F::cast_from(3.0_f64) * t19775;
    let t19778 = t83 * t1008 * t5075;
    let t19779 = t1673 * t568;
    let t19783 = -F::cast_from(9.0_f64) * t1535 * t17258 * t2537 + F::cast_from(9.0_f64) * t1535 * t19770 * t568 + F::cast_from(18.0_f64) * t1535 * t19779 * t7197 + F::cast_from(9.0_f64) * t1535 * t2575 * t5191 + F::cast_from(6.0_f64) * t1673 * t19766 * t2536 + t16626 + t19755 - t19757 + t19759 + t19776 + t19778;
    (t19776, t19778, t19783)
}
