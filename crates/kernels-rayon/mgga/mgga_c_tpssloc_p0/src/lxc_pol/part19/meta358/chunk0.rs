//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1299/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1299(t2884: f64, t302: f64, t2887: f64, t10727: f64, t10817: f64, t10655: f64, t10731: f64, t10661: f64, t2836: f64, t2845: f64, t10697: f64, t2792: f64, t912: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t42224 = t2884 * t2884;
    let t42226 = t302 / t42224;
    let t42227 = t2887 * t2887;
    let t42228 = 1.0_f64 / t42227;
    let t42233 = 24.0_f64 * t10817 * t10727;
    let t42235 = 0.1929837539843104208e3_f64 * t10655 * t10731;
    let t42238 = 0.57895126195293126241e3_f64 * t10661 * t2845 * t2836;
    let t42241 = 8.0_f64 * t2792 * t10697 * t912;
    (t42226, t42228, t42233, t42235, t42238, t42241)
}
