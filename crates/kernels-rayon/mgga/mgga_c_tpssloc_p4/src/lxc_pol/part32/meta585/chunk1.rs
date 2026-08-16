//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1969/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1969(t29748: f64, t29793: f64, t1241: f64, t2154: f64, t6243: f64, t11606: f64, t24615: f64, t7300: f64, t1409: f64, t1760: f64, t24602: f64, t24601: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t29794 = t29748 + t29793;
    let t29795 = t1241 * t29794;
    let t29797 = t2154 * t6243;
    let t29798 = t11606 * t29797;
    let t29803 = t24615 * t6243;
    let t29804 = t7300 * t29803;
    let t29808 = t24602 * t1409 * t1760;
    let t29809 = t24601 * t29808;
    (t29794, t29795, t29798, t29803, t29804, t29808, t29809)
}
