//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 739/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk739(t6706: f64, t6964: f64, t2471: f64, t4399: f64, t1305: f64, t487: f64, t2365: f64, t1416: f64, t4803: f64, t586: f64, t2479: f64, t1065: f64, t2465: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6965 = t6964 * t6706;
    let t6968 = t4399 * t2471;
    let t6970 = t487 * t1305;
    let t6971 = t2365 * t6970;
    let t6972 = t1416 * t6971;
    let t6974 = t4803 * t586;
    let t6975 = t6974 * t2479;
    let t6977 = t2465 * t1065;
    (t6965, t6968, t6972, t6974, t6975, t6977)
}
