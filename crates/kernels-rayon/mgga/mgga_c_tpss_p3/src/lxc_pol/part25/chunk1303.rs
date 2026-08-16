//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1303/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1303(t4674: f64, t623: f64, t13546: f64, t93: f64, t1976: f64, t4573: f64, t4570: f64, t615: f64, t77: f64, t10289: f64, t1290: f64, t3418: f64, t3426: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t69069 = t623 * t4674;
    let t69072 = t93 * t13546;
    let t69087 = t1976 * t4573;
    let t69097 = t77 * t615 * t4570;
    let t69108 = t10289 * t1290;
    let t69111 = t3418 * t3426;
    (t69069, t69072, t69087, t69097, t69108, t69111)
}
