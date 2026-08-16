//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1018/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1018(t11004: f64, t10982: f64, t1289: f64, t8493: f64, t1985: f64, t8609: f64, t128: f64) -> (f64, f64, f64, f64) {
    let t11005 = 4.0_f64 / 9.0_f64 * t11004;
    let t11006 = 2.0_f64 / 9.0_f64 * t10982;
    let t11007 = t8493 * t1289;
    let t11008 = t11007 * t1985;
    let t11009 = t8609 * t11008;
    let t11010 = t128 * t11009;
    (t11005, t11006, t11008, t11010)
}
