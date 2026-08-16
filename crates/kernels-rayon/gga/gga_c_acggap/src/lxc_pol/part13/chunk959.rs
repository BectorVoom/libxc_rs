//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 959/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk959(t1967: f64, t7523: f64, t7535: f64, t2104: f64, t7610: f64, t1988: f64, t7472: f64, t1113: f64, t7736: f64, t1098: f64, t7605: f64, t3445: f64, t7647: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t31845 = t1967 * t7523;
    let t31847 = t1967 * t7535;
    let t31849 = t7610 * t2104;
    let t31851 = t1988 * t7472;
    let t31855 = t7736 * t1113;
    let t31857 = t7605 * t1098;
    let t31859 = t7647 * t3445;
    (t31845, t31847, t31849, t31851, t31855, t31857, t31859)
}
