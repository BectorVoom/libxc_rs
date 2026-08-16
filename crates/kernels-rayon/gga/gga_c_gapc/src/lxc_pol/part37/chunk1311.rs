//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1311/1445 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1311(t11216: f64, t13646: f64, t520: f64, t13654: f64, t35541: f64, t3948: f64, t11258: f64, t2932: f64, t3946: f64, t1006: f64, t3639: f64, t4026: f64) -> (f64, f64, f64, f64) {
    let t35650 = t11216 * t520 * t13646;
    let t35653 = t35541 * t3948 * t13654;
    let t35656 = t2932 * t3946 * t11258;
    let t35659 = t1006 * t3639 * t4026;
    (t35650, t35653, t35656, t35659)
}
