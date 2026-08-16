//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1021/1378 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1021(t1066: f64, t23485: f64, t247: f64, t1651: f64, t5819: f64, t4801: f64, t1042: f64, t1668: f64, t6305: f64, t373: f64, t11257: f64, t11506: f64, t23451: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t23630 = t247 * t1066 * t23485;
    let t23633 = t5819 * t1651;
    let t23634 = t4801 * t23633;
    let t23635 = t1042 * t23634;
    let t23640 = t6305 * t1668;
    let t23641 = t373 * t23640;
    let t23642 = t23641 * t11257;
    let t23643 = t1042 * t23642;
    let t23648 = t11506 * t23451;
    (t23630, t23633, t23635, t23640, t23641, t23643, t23648)
}
