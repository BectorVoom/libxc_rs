//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 742/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk742(t3781: f64, t847: f64, t1415: f64, t2504: f64, t849: f64, t854: f64, t1421: f64, t673: f64, t2515: f64, t3749: f64, t141: f64, t3754: f64, t861: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t3782 = t847 * t3781;
    let t3789 = t2504 * t1415;
    let t3790 = t3789 * t849;
    let t3792 = t854 * t3781;
    let t3795 = t673 * t1421;
    let t3797 = t2515 * t3749;
    let t3798 = t141 * t3797;
    let t3800 = t861 * t3754;
    (t3782, t3789, t3790, t3792, t3795, t3797, t3798, t3800)
}
