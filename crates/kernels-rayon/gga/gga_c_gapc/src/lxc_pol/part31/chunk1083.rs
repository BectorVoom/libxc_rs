//! GGA_C_GAPC lxc pol — lxc_pol part 31 (v4rho2sigma2_10) CSE chunk 1083/1447 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part31_v4rho2sigma2_10_chunk1083(t6: f64, t8140: f64, t8139: f64, t11925: f64, t286: f64, t2206: f64, t869: f64, t186: f64, t2674: f64, t1087: f64, t2188: f64, t2254: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15679 = t8140 * t6;
    let t15680 = t8139 * t15679;
    let t15699 = t11925 * t286;
    let t15805 = t869 * t2206;
    let t15811 = t2674 * t186;
    let t15835 = t1087 * t2188;
    let t15843 = t1087 * t2254;
    (t15679, t15680, t15699, t15805, t15811, t15835, t15843)
}
