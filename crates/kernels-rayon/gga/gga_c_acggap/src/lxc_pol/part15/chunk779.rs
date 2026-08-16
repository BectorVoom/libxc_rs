//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 779/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk779(t2031: f64, t507: f64, t2030: f64, t2061: f64, t2060: f64, t2314: f64, t7447: f64, t527: f64, t7685: f64, t1426: f64, t2085: f64, t535: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t8823 = t507 * t2031;
    let t8824 = t2030 * t8823;
    let t8826 = t507 * t2061;
    let t8827 = t2060 * t8826;
    let t8829 = t7447 * t2314;
    let t8835 = t7685 * t527;
    let t8838 = t1426 * t535 * t2085;
    (t8823, t8824, t8826, t8827, t8829, t8835, t8838)
}
