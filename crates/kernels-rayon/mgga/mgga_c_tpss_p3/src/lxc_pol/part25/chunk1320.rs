//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1320/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1320(t17974: f64, t4775: f64, t4771: f64, t14193: f64, t17964: f64, t14197: f64, t14202: f64, t14212: f64, t63920: f64, t14216: f64, t19703: f64, t14171: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t69952 = t17974 * t4775;
    let t69954 = t17974 * t4771;
    let t69956 = t17964 * t14193;
    let t69958 = t17964 * t14197;
    let t69960 = t17964 * t14202;
    let t69962 = t63920 * t14212;
    let t69964 = t19703 * t14216;
    let t69966 = t17964 * t14171;
    (t69952, t69954, t69956, t69958, t69960, t69962, t69964, t69966)
}
