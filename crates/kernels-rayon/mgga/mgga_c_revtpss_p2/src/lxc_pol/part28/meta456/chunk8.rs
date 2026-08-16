//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 1739/2277 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1739(t17974: f64, t3575: f64, t17807: f64, t225: f64, t494: f64, t1209: f64, t488: f64, t1828: f64, t3736: f64, t3790: f64, t3737: f64, t1811: f64, t3566: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t17975 = t17974 * t3575;
    let t17979 = t17807 * t225 * t494;
    let t17986 = t1209 * t488;
    let t17987 = t3736 * t1828;
    let t17988 = t17987 * t3575;
    let t17991 = t1828 * t3790;
    let t17992 = t3737 * t17991;
    let t17995 = t3566 * t1811;
    (t17975, t17979, t17986, t17988, t17992, t17995)
}
