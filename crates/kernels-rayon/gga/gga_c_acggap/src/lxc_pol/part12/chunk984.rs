//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 984/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk984(t315: f64, t323: f64, t8301: f64, t29979: f64, t29980: f64, t638: f64, t15758: f64, t32041: f64, t8306: f64, t2176: f64, t3883: f64, t32142: f64, t8085: f64) -> (f64, f64, f64, f64, f64) {
    let t33147 = t315 * t8301 * t323;
    let t33150 = t29979 * t638 * t29980;
    let t33153 = t32041 * t8306 * t15758;
    let t33155 = t2176 * t3883;
    let t33157 = t32142 * t8085;
    (t33147, t33150, t33153, t33155, t33157)
}
