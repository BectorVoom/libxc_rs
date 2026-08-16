//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 985/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk985(t22642: f64, t22690: f64, t31618: f64, t115384: f64, t1992: f64, t22897: f64, t3792: f64, t22751: f64, t31620: f64, t552: f64, t7191: f64, t1307: f64, t6637: f64, t6888: f64) -> (f64, f64, f64, f64) {
    let t115390 = t22642 * t22690 * t31618;
    let t115391 = 0.82246703342411321824e-2_f64 * t115390;
    let t115395 = t1992 * t22897 * t115384 * t3792;
    let t115397 = t22751 * t31620;
    let t115399 = t552 * t7191;
    let t115402 = t6888 * t6637 * t115399 * t1307;
    (t115391, t115395, t115397, t115402)
}
