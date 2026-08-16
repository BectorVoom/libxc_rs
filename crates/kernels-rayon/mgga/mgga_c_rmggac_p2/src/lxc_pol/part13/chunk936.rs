//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 936/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk936(t2289: f64, t36542: f64, t34884: f64, t8668: f64, t8831: f64, t8836: f64, t8843: f64, t2320: f64, t35151: f64, t34847: f64, t1525: f64, t236: f64, t498: f64, t7230: f64, t7231: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t40556 = t36542 * t2289;
    let t40558 = t34884 * t8668;
    let t40560 = t34884 * t8831;
    let t40562 = t34884 * t8836;
    let t40564 = t34884 * t8843;
    let t40566 = t35151 * t2320;
    let t40568 = t34847 * t8668;
    let t40573 = t7230 * t7231 * t236 * t1525 * t498;
    (t40556, t40558, t40560, t40562, t40564, t40566, t40568, t40573)
}
