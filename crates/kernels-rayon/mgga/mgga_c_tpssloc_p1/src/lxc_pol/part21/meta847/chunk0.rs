//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3063/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3063(t18834: f64, t3315: f64, t1117: f64, t3313: f64, t18258: f64, t3307: f64, t1147: f64, t18710: f64, t3400: f64, t6063: f64, t1157: f64, t15121: f64, t15133: f64, t1695: f64, t18899: f64, t3396: f64, t3404: f64, t44300: f64, t4835: f64, t4858: f64, t51366: f64, t6056: f64, t63563: f64, t63566: f64, t63568: f64, t63571: f64, t63574: f64, t63576: f64, t63579: f64, t63582: f64, t63585: f64, t63587: f64) -> (f64, f64, f64) {
    let t63588 = t18834 * t3315;
    let t63591 = 0.32163958997385070134e2_f64 * t3313 * t63588 * t1117;
    let t63594 = 0.16081979498692535067e2_f64 * t3313 * t18258 * t3307;
    let t63597 = t18710 * t1147;
    let t63602 = t6063 * t3400;
    let t63611 = t63563 + t63566 + t63568 + t63571 + t63574 + t63576 + t63579 + t63582 + t63585 - t63587 - t63591 - t63594 + 0.32163958997385070134e2_f64 * t44300 * t6056 + 0.11696447245269292414e1_f64 * t63597 * t1157 + 0.5848223622634646207e0_f64 * t18899 * t3396 + 0.17315859105681463759e2_f64 * t63602 * t3404 + 0.11696447245269292414e1_f64 * t51366 * t1695 + 0.23392894490538584828e1_f64 * t15121 * t4858 + 0.11696447245269292414e1_f64 * t4835 * t15133;
    (t63591, t63594, t63611)
}
