//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 723/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk723(t1404: f64, t2880: f64, t120: f64, t118: f64, t1803: f64, t61: f64, t1504: f64, t1461: f64, t4043: f64, t1030: f64, t3141: f64, t5059: f64) -> (f64, f64, f64, f64, f64) {
    let t8585 = t2880 * t1404;
    let t8586 = t120 * t8585;
    let t8588 = t1803 * t118;
    let t8589 = t61 * t8588;
    let t8590 = t2880 * t1504;
    let t8591 = t8589 * t8590;
    let t8619 = t1461 * t4043;
    let t8620 = t1030 * t8619;
    let t8621 = t3141 * t5059;
    (t8586, t8591, t8619, t8620, t8621)
}
