//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1182/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1182(t11964: f64, t26447: f64, t29314: f64, t311: f64, t1971: f64, t9244: f64, t1084: f64, t9929: f64, t11910: f64, t30095: f64, t2562: f64, t7120: f64) -> (f64, f64, f64, f64, f64) {
    let t33893 = t311 * t11964 * t26447 * t29314;
    let t33895 = t1971 * t9244;
    let t33897 = t1084 * t33895 * t9929;
    let t33899 = t11910 * t30095;
    let t33901 = t7120 * t2562;
    (t33893, t33895, t33897, t33899, t33901)
}
