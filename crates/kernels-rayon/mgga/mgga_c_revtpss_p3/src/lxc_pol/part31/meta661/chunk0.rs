//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2239/2259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2239(t1353: f64, t30122: f64, t28167: f64, t8717: f64, t2014: f64, t25190: f64, t29494: f64, t27833: f64, t7901: f64, t28020: f64, t5542: f64, t1450: f64, t21969: f64) -> (f64, f64, f64, f64, f64) {
    let t109104 = t30122 * t1353;
    let t109107 = 12.0_f64 * t28167 * t8717 * t109104;
    let t109110 = 3.0_f64 * t2014 * t25190 * t29494;
    let t109112 = 6.0_f64 * t27833 * t7901;
    let t109117 = 2.0_f64 * t2014 * t28020 * t5542;
    let t109118 = t1450 * t21969;
    (t109107, t109110, t109112, t109117, t109118)
}
