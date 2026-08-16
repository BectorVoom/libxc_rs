//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 713/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk713(t5: f64, t1317: f64, t1981: f64, t3418: f64, t4566: f64, t4570: f64, t4626: f64, t578: f64, t91: f64, t117: f64) -> (f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t4630 = piecewise3(t8, 0.0_f64, -8.0_f64 * t1317 * t3418 + 20.0_f64 * t1981 * t4570 + t4566 * t91 - 4.0_f64 * t4626 * t578);
    let t4631 = t4630 * t117;
    (t4630, t4631)
}
