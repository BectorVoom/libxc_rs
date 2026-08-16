//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1264/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1264(t3566: f64, t8190: f64, t5251: f64, t8945: f64, t26921: f64, t8205: f64, t17306: f64, t2142: f64, t12587: f64, t8220: f64, t116: f64, t30004: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t105512 = t3566 * t8190;
    let t105530 = t5251 * t8945;
    let t105558 = t8205 * t26921;
    let t105579 = t17306 * t2142;
    let t105669 = t8220 * t12587;
    let t105819 = t116 * t30004;
    (t105512, t105530, t105558, t105579, t105669, t105819)
}
