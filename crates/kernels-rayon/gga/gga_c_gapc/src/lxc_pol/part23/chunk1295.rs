//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 1295/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk1295(t11629: f64, t11637: f64, t1061: f64, t23523: f64, t3728: f64, t6927: f64, t2212: f64, t3738: f64, t6791: f64, t11626: f64, t6179: f64, t824: f64) -> (f64, f64, f64, f64) {
    let t35966 = t11637 * t11629;
    let t35970 = t1061 * t23523 * t3728 * t6927;
    let t35973 = t3738 * t6791 * t2212;
    let t35976 = t824 * t6179 * t11626;
    (t35966, t35970, t35973, t35976)
}
