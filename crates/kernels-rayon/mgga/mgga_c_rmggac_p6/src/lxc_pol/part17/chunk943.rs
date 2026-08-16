//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 943/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk943(t8422: f64, t8577: f64, t8427: f64, t8432: f64, t8437: f64, t40661: f64, t8443: f64, t2001: f64, t2281: f64, t326: f64, t558: f64, t7720: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t45670 = t8577 * t8422;
    let t45672 = t8577 * t8427;
    let t45674 = t8577 * t8432;
    let t45676 = t8577 * t8437;
    let t45678 = t40661 * t8443;
    let t45685 = t2001 * t326 * t2281 * t558;
    let t45686 = t7720 * t45685;
    (t45670, t45672, t45674, t45676, t45678, t45686)
}
