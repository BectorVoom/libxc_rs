//! GGA_C_GAPC lxc pol — lxc_pol part 26 (v4rho2sigma2_5) CSE chunk 1260/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part26_v4rho2sigma2_5_chunk1260(t11243: f64, t8489: f64, t11195: f64, t24980: f64, t152: f64, t515: f64, t2903: f64, t623: f64, t3945: f64, t11189: f64, t1845: f64, t996: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t35564 = t8489 * t11243;
    let t35566 = t24980 * t11195;
    let t35568 = t515 * t152;
    let t35570 = t2903 * t35568 * t623;
    let t35572 = t3945 * t11195;
    let t35575 = t996 * t1845 * t11189;
    (t35564, t35566, t35568, t35570, t35572, t35575)
}
