//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1141/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1141(t19775: f64, t1008: f64, t5075: f64, t83: f64, t1673: f64, t568: f64, t1535: f64, t16626: f64, t17258: f64, t19755: f64, t19757: f64, t19759: f64, t19766: f64, t19770: f64, t2536: f64, t2537: f64, t2575: f64, t5191: f64, t7197: f64) -> (f64, f64, f64) {
    let t19776 = 3.0_f64 * t19775;
    let t19778 = t83 * t1008 * t5075;
    let t19779 = t1673 * t568;
    let t19783 = -9.0_f64 * t1535 * t17258 * t2537 + 9.0_f64 * t1535 * t19770 * t568 + 18.0_f64 * t1535 * t19779 * t7197 + 9.0_f64 * t1535 * t2575 * t5191 + 6.0_f64 * t1673 * t19766 * t2536 + t16626 + t19755 - t19757 + t19759 + t19776 + t19778;
    (t19776, t19778, t19783)
}
