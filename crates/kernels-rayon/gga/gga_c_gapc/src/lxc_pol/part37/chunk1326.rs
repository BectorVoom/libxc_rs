//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1326/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1326(t10357: f64, t11674: f64, t35734: f64, t190: f64, t24086: f64, t35729: f64, t6852: f64, t10256: f64, t11663: f64, t2229: f64, t3729: f64, t11670: f64, t828: f64) -> (f64, f64, f64, f64, f64) {
    let t35861 = t35734 * t11674 * t10357;
    let t35865 = t35729 * t6852 * t190 * t24086;
    let t35867 = t10256 * t11663;
    let t35869 = t2229 * t3729;
    let t35871 = t828 * t11670;
    (t35861, t35865, t35867, t35869, t35871)
}
