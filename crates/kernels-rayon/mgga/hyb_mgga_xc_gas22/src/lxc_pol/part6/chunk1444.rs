//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 1444/1455 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_rkernel_math::erf::{erf_approx};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk1444(t1145: f64, t2876: f64, t4544: f64, t4540: f64, t2884: f64, t4535: f64, t30771: f64, t9561: f64, t26728: f64, t2869: f64, t30596: f64, t30670: f64, t30772: f64, t30784: f64, t30860: f64, t3720: f64, t3739: f64, t3747: f64, t7734: f64, t7739: f64, t7769: f64, t7775: f64, t7780: f64, t9527: f64, t9594: f64, t9636: f64) -> (f64, f64, f64) {
    let t31410 = t1145 * t4544 * t2876;
    let t31414 = t1145 * t4540 * t2876;
    let t31419 = t2884 * t4535;
    let t31436 = t9561 * t30771;
    let t31441 = -168.0_f64 * t7780 * t31410 + 6.0_f64 * t7739 * t31414 - 12.0_f64 * t7769 * t31410 - 6400.0_f64 / 27.0_f64 * t3720 * t31419 + 60.0_f64 * t7775 * t1145 * t4540 * t2869 + 126.0_f64 * t7734 * t31414 - 3200.0_f64 / 9.0_f64 * t26728 * t30596 + 3200.0_f64 / 9.0_f64 * t30670 * t9636 - 16.0_f64 / 3.0_f64 * t9527 * t30784 + 5632.0_f64 / 2187.0_f64 * t9594 * t30772 + 1408.0_f64 / 243.0_f64 * t3739 * t31436 + 704.0_f64 / 81.0_f64 * t3747 * t30860;
    (t31419, t31436, t31441)
}
