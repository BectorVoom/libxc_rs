//! GGA_C_GAPC lxc pol — lxc_pol part 21 (v4rho2sigma2_0) CSE chunk 1001/1307 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part21_v4rho2sigma2_0_chunk1001(t186: f64, t2786: f64, t2579: f64, t923: f64, t6: f64, t8140: f64, t8139: f64, t11925: f64, t286: f64, t2206: f64, t869: f64, t2674: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15644 = t2786 * t186;
    let t15650 = t2579 * t923;
    let t15679 = t8140 * t6;
    let t15680 = t8139 * t15679;
    let t15699 = t11925 * t286;
    let t15805 = t869 * t2206;
    let t15811 = t2674 * t186;
    (t15644, t15650, t15679, t15680, t15699, t15805, t15811)
}
