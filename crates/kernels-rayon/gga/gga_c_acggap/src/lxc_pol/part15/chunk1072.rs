//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1072/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1072(t32041: f64, t36019: f64, t8306: f64, t32181: f64, t36475: f64, t38086: f64, t2385: f64, t310: f64, t464: f64, t9369: f64, t2131: f64, t2147: f64, t309: f64, t9413: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t38215 = t32041 * t8306 * t36019;
    let t38224 = t32181 * t38086 * t36475;
    let t38226 = t310 * t2385;
    let t38228 = 0.13170898365871023197e1_f64 * t38226 * t464;
    let t38232 = 0.13170898365871023197e1_f64 * t310 * t9369;
    let t38241 = 0.34694512752820797848e1_f64 * t2131 * t2147 * t9413 * t309;
    (t38215, t38224, t38226, t38228, t38232, t38241)
}
