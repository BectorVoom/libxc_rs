//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 452/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk452(t236: f64, t9211: f64, t321: f64, t618: f64, t2313: f64, t7715: f64, t674: f64, t2228: f64, t558: f64, t1587: f64, t699: f64, t1652: f64, t2211: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9212 = t236 * t9211;
    let t9216 = t618 * t321;
    let t9217 = t236 * t9216;
    let t9221 = t2313 * t7715;
    let t9222 = t9221 * t674;
    let t9302 = t2228 * t558;
    let t9332 = t699 * t1587;
    let t9340 = t2211 * t1652;
    (t9212, t9217, t9221, t9222, t9302, t9332, t9340)
}
