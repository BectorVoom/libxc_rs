//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 717/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk717(t5: f64, t67: f64, t7254: f64, t1864: f64, t2109: f64, t6509: f64, t1860: f64, t2110: f64, t6486: f64, t6492: f64, t6495: f64, t7246: f64, t112: f64) -> (f64, f64, f64, f64, f64) {
    let t7 = piecewise3(0.0_f64 < t5, t5, -t5);
    let t8 = -t7 <= -0.999999999999e0_f64;
    let t7255 = t7254 * t67;
    let t7256 = t7255 * t1864;
    let t7259 = t2109 * t6509;
    let t7263 = piecewise3(t8, 0.0_f64, -t6486 * t2110 / 6.0_f64 + 5.0_f64 / 6.0_f64 * t7246 * t6492 + t6495 * t2110 / 3.0_f64 - t1860 * t7256 / 6.0_f64 - t1860 * t7259 / 6.0_f64);
    let t7264 = t7263 * t112;
    (t7255, t7256, t7259, t7263, t7264)
}
