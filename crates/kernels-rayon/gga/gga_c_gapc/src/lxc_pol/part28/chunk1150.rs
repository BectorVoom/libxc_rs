//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1150/1429 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1150(t11356: f64, t3363: f64, t9865: f64, t33211: f64, t7595: f64, t28602: f64, t3784: f64, t3131: f64, t8785: f64, t1084: f64, t15610: f64, t1734: f64, t8709: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33405 = t3363 * t11356 * t9865;
    let t33407 = t33211 * t7595;
    let t33409 = t3784 * t28602;
    let t33411 = t3131 * t8785;
    let t33413 = t1084 * t33411 * t15610;
    let t33415 = t1734 * t8709;
    (t33405, t33407, t33409, t33411, t33413, t33415)
}
