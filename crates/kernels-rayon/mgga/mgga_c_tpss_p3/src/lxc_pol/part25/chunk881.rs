//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 881/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk881(t7921: f64, t7997: f64, t162: f64, t158: f64, t2206: f64, t2218: f64, t713: f64, t720: f64, t7870: f64, t735: f64, t2214: f64, t7813: f64, t7857: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t7998 = t7921 + t7997;
    let t7999 = t162 * t7998;
    let t8000 = t158 * t7999;
    let t8006 = t2218 * t2206;
    let t8017 = t713 * t7870 * t720;
    let t8019 = 0.5848223622634646207e0_f64 * t735 * t8017;
    let t8021 = t7857 * t7813 * t2214;
    (t7998, t8000, t8006, t8017, t8019, t8021)
}
