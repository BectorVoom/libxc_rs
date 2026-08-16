//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1078/1414 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1078(t1956: f64, t2234: f64, t6675: f64, t732: f64, t22410: f64, t22417: f64, t22434: f64, t22439: f64, t22646: f64, t22648: f64, t22652: f64, t22655: f64, t22657: f64, t22659: f64, t22661: f64) -> (f64, f64, f64) {
    let t23404 = t2234 * t1956;
    let t23406 = t732 * t6675;
    let t23409 = -t22410 + t22646 - t22648 - t22652 - t22655 - t22417 + t22434 - t22439 + t22657 - t22659 + t22661;
    (t23404, t23406, t23409)
}
