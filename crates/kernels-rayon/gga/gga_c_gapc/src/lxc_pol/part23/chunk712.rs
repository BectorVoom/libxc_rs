//! GGA_C_GAPC lxc pol — lxc_pol part 23 (v4rho2sigma2_2) CSE chunk 712/1308 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part23_v4rho2sigma2_2_chunk712(t1437: f64, t8416: f64, t2902: f64, t424: f64, t2915: f64, t116: f64, t1474: f64, t152: f64, t188: f64, t505: f64, t1947: f64, t473: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t8417 = t1437 * t8416;
    let t8419 = t2902 * t424;
    let t8420 = t8419 * t2915;
    let t8422 = t1474 * t116;
    let t8423 = t8422 * t2915;
    let t8426 = t188 * t505 * t152;
    let t8427 = t8426 * t1947;
    let t8428 = t473 * t8427;
    (t8417, t8419, t8420, t8422, t8423, t8428)
}
