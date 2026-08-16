//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 308/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk308(t220: f64, t368: f64, t975: f64, t983: f64, t984: f64, t985: f64, t981: f64, t373: f64, t976: f64, t978: f64, t375: f64) -> (f64, f64, f64, f64) {
    let t990 = t220 * t368 * t975 + t983 * t984 * t985;
    let t991 = t981 * t990;
    let t993 = t373 * t976 - t978 * t991;
    let t995 = 1.0_f64 / t375;
    (t990, t991, t993, t995)
}
