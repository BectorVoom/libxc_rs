//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 640/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk640(t3894: f64, t914: f64, t1457: f64, t2629: f64, t1448: f64, t2593: f64, t905: f64, t912: f64, t3882: f64, t895: f64, t904: f64, t2618: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3896 = 0.5848223622634646207e0_f64 * t3894 * t914;
    let t3898 = 0.5848223622634646207e0_f64 * t2629 * t1457;
    let t3899 = t2593 * t1448;
    let t3900 = t3899 * t905;
    let t3902 = 0.11696447245269292414e1_f64 * t912 * t3900;
    let t3904 = t895 * t3882 * t904;
    let t3906 = 0.5848223622634646207e0_f64 * t912 * t3904;
    let t3907 = t2618 * t1448;
    (t3896, t3898, t3899, t3900, t3902, t3904, t3906, t3907)
}
