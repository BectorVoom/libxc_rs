//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 945/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk945(t29976: f64, t8337: f64, t29979: f64, t29980: f64, t638: f64, t15758: f64, t32041: f64, t8306: f64, t32142: f64, t8085: f64, t2217: f64, t394: f64) -> (f64, f64, f64, f64, f64) {
    let t33120 = t29976 * t8337;
    let t33150 = t29979 * t638 * t29980;
    let t33153 = t32041 * t8306 * t15758;
    let t33157 = t32142 * t8085;
    let t33175 = t394 * t2217;
    (t33120, t33150, t33153, t33157, t33175)
}
