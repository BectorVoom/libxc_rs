//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 968/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk968(t7941: f64, t862: f64, t157: f64, t406: f64, t847: f64, t309: f64, t929: f64, t1679: f64, t811: f64, t9460: f64, t2248: f64, t469: f64) -> (f64, f64, f64, f64, f64) {
    let t32181 = t862 * t7941;
    let t32194 = t847 * t406 * t157;
    let t32199 = t309 * t929 * t157;
    let t32257 = t1679 * t9460 * t811;
    let t32262 = t2248 * t469;
    (t32181, t32194, t32199, t32257, t32262)
}
