//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2606/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2606(t1227: f64, t13969: f64, t15649: f64, t43763: f64, t44827: f64, t11539: f64, t1174: f64, t14740: f64, t11694: f64, t11739: f64, t1215: f64, t1653: f64, t3493: f64, t3577: f64, t3578: f64, t44936: f64, t45119: f64, t45128: f64, t45197: f64, t4582: f64, t4728: f64, t48497: f64, t52183: f64, t52704: f64, t52893: f64, t52897: f64, t52903: f64, t52906: f64, t52908: f64, t52911: f64) -> f64 {
    let t52917 = t1227 * t13969 * t15649;
    let t52919 = t44827 * t43763;
    let t52926 = t1174 * t11539 * t14740;
    let t52928 = -t45119 * t3578 * t1653 * t11739 / 4608.0_f64 - 5.0_f64 / 1728.0_f64 * t52893 * t45128 * t52183 + 3.0_f64 / 512.0_f64 * t45197 * t52897 * t52704 * t3493 * t1215 - t52903 * t11694 / 288.0_f64 - t52906 / 144.0_f64 + t52908 / 768.0_f64 - t3577 * t3578 * t4728 * t52911 / 768.0_f64 - t52917 / 576.0_f64 + 55.0_f64 / 15552.0_f64 * t1227 * t4582 * t52919 * t48497 + t44936 / 108.0_f64 + t52926 / 216.0_f64;
    t52928
}
