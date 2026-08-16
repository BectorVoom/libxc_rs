//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 54 (v4rho2sigma2_10) CSE chunk 1405/1484 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part54_v4rho2sigma2_10_chunk1405(t100688: f64, t101840: f64, t119746: f64, t119780: f64, t121264: f64, t1877: f64, t24191: f64, t2522: f64, t25901: f64, t25930: f64, t26744: f64, t26756: f64, t30974: f64, t31434: f64, t31441: f64, t31448: f64, t31502: f64, t33483: f64, t33537: f64, t7114: f64, t8566: f64, t89849: f64, t89992: f64, t92271: f64) -> f64 {
    let t121982 = t26756 * t89849 * t33483 + 3.0_f64 / 2.0_f64 * t2522 * t8566 * t25901 + t101840 * t31502 - t1877 * t7114 * t119746 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t24191 * t89992 * t31441 - t1877 * t26744 * t30974 / 2.0_f64 + t92271 * t33537 + t121264 + t26756 * t100688 * t31448 - t1877 * t31434 * t25930 / 2.0_f64 - 3.0_f64 / 2.0_f64 * t24191 * t119780;
    t121982
}
