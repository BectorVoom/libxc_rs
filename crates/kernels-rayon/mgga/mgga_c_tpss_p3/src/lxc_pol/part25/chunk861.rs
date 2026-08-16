//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 861/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk861(t1980: f64, t574: f64, t88: f64, t89: f64, t90: f64, t29: f64) -> (f64, f64, f64) {
    let t7682 = t574 * t1980;
    let t7689 = 1.0_f64 / t90 / t89 / t88;
    let t7690 = t29 * t7689;
    (t7682, t7689, t7690)
}
