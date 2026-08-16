//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 750/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk750(t3904: f64, t912: f64, t1448: f64, t2618: f64, t2621: f64, t903: f64, t140: f64, t1460: f64, t925: f64, t2697: f64, t926: f64, t3749: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3906 = 0.5848223622634646207e0_f64 * t912 * t3904;
    let t3907 = t2618 * t1448;
    let t3908 = t2621 * t903;
    let t3909 = t3907 * t3908;
    let t3911 = 0.17315859105681463759e2_f64 * t912 * t3909;
    let t3916 = t140 * t1460;
    let t3917 = t925 * t3916;
    let t3919 = t926 * t2697;
    let t3920 = t3919 * t3749;
    (t3906, t3907, t3908, t3909, t3911, t3916, t3917, t3919, t3920)
}
