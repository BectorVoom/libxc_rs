//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 829/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk829(t1324: f64, t999: f64, t1323: f64, t2778: f64, t1064: f64, t1328: f64, t2854: f64, t6320: f64, t2787: f64, t4324: f64, t2343: f64, t2765: f64, t4807: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7952 = t999 * t1324;
    let t7957 = t2778 * t1323;
    let t7958 = t1064 * t7957;
    let t7963 = t2854 * t1328;
    let t7964 = t6320 * t7963;
    let t7967 = t2787 * t4324;
    let t7968 = t2343 * t7967;
    let t7971 = t2765 * t4807;
    (t7952, t7957, t7958, t7963, t7964, t7967, t7968, t7971)
}
