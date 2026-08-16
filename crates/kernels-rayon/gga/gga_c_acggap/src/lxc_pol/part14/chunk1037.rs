//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1037/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1037(t2138: f64, t2147: f64, t322: f64, t8392: f64, t7998: f64, t8397: f64, t1659: f64, t7973: f64, t1539: f64, t309: f64, t32181: f64, t36433: f64) -> (f64, f64, f64, f64) {
    let t36452 = 0.34694512752820797848e1_f64 * t2138 * t2147 * t8392 * t322;
    let t36460 = t8397 * t7998;
    let t36473 = 0.13170898365871023197e1_f64 * t7973 * t1659;
    let t36475 = t1539 * t309;
    let t36477 = t32181 * t36433 * t36475;
    (t36452, t36460, t36473, t36477)
}
