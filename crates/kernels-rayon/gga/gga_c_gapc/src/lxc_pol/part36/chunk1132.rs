//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 1132/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk1132(t15853: f64, t17874: f64, t311: f64, t4043: f64, t519: f64, t7113: f64, t7547: f64, t7549: f64, t1882: f64, t277: f64, t9959: f64, t11954: f64, t2981: f64, t876: f64) -> (f64, f64, f64, f64) {
    let t33988 = t311 * t15853 * t4043 * t519 * t17874;
    let t33991 = t7547 * t7113 * t7549;
    let t33998 = t277 * t1882 * t9959;
    let t34001 = t11954 * t2981 * t876;
    (t33988, t33991, t33998, t34001)
}
