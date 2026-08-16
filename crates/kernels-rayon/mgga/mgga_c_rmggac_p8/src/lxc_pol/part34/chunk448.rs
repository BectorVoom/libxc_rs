//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 448/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk448(t236: f64, t495: f64, t551: f64, t618: f64, t117: f64, t6477: f64, t5888: f64, t875: f64, t1475: f64, t209: f64, t476: f64, t558: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9117 = t236 * t551 * t495;
    let t9122 = t236 * t618 * t495;
    let t9128 = t6477 * t117;
    let t9137 = t875 * t5888;
    let t9145 = t1475 * t495;
    let t9146 = t236 * t9145;
    let t9151 = t551 * t476 * t209;
    let t9152 = t236 * t9151;
    let t9157 = t558 * t476 * t209;
    (t9117, t9122, t9128, t9137, t9145, t9146, t9151, t9152, t9157)
}
