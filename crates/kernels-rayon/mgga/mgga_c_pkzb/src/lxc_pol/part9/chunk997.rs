//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 997/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk997(t2209: f64, t3041: f64, t2215: f64, t3046: f64, t836: f64, t3052: f64, t218: f64, t3061: f64, t675: f64, t3065: f64, t1167: f64, t2185: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7970 = t3041 * t2209;
    let t7972 = t2215 * t3046;
    let t7973 = t7972 * t836;
    let t7975 = t3052 * t2209;
    let t7979 = t218 * t675 * t3061;
    let t7980 = 0.32862666666666666666e0_f64 * t7979;
    let t7982 = t218 * t675 * t3065;
    let t7983 = 0.32862666666666666666e0_f64 * t7982;
    let t7984 = t2185 * t1167;
    (t7970, t7972, t7973, t7975, t7979, t7980, t7982, t7983, t7984)
}
