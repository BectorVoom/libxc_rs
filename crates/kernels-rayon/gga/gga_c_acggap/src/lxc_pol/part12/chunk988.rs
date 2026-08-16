//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 988/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk988(t309: f64, t8306: f64, t32130: f64, t7934: f64, t32003: f64, t322: f64, t3919: f64, t8347: f64, t29991: f64, t639: f64, t8114: f64, t872: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t33232 = t8306 * t309;
    let t33234 = t32130 * t33232 * t7934;
    let t33240 = t32003 * t8306 * t322 * t7934;
    let t33250 = t8347 * t3919;
    let t33256 = t29991 * t639;
    let t33258 = t8114 * t872;
    (t33232, t33234, t33240, t33250, t33256, t33258)
}
