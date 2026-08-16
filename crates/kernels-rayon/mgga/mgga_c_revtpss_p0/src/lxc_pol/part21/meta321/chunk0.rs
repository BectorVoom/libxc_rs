//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 1597/3259 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk1597(t2453: f64, t861: f64, t2458: f64, t2761: f64, t786: f64, t789: f64, t212: f64, t2760: f64, t780: f64, t689: f64, t785: f64, t860: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t11018 = t2453 * t861;
    let t11019 = t11018 * t2458;
    let t11021 = t786 * t2761;
    let t11022 = t11021 * t789;
    let t11024 = t212 * t2760;
    let t11025 = t11024 * t780;
    let t11026 = t689 * t11025;
    let t11028 = t785 * t860;
    (t11018, t11019, t11021, t11022, t11024, t11025, t11026, t11028)
}
