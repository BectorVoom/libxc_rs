//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1261/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1261(t26114: f64, t8675: f64, t26179: f64, t31908: f64, t7458: f64, t191: f64, t192: f64, t27903: f64, t2020: f64, t104977: f64, t1874: f64, t27863: f64, t6525: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t123093 = t26114 * t8675;
    let t123095 = t26179 * t8675;
    let t123097 = t7458 * t31908;
    let t123111 = t27903 * t191 * t192;
    let t123112 = t123111 * t2020;
    let t123113 = t104977 * t1874;
    let t123115 = t27863 * t6525;
    (t123093, t123095, t123097, t123112, t123113, t123115)
}
