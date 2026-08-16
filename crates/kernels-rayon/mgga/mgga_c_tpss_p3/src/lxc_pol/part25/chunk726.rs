//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 726/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk726(t219: f64, t4747: f64, t2357: f64, t4706: f64, t4701: f64, t778: f64, t1373: f64, t1375: f64, t222: f64, t224: f64) -> (f64, f64, f64, f64) {
    let t4748 = t4747 * t219;
    let t4752 = t2357 * t4706;
    let t4755 = t778 * t4701;
    let t4758 = 6.0_f64 * t1373 * t1375 - 12.0_f64 * t222 * t4752 + 3.0_f64 * t222 * t4755 - t224 * t4748;
    (t4748, t4752, t4755, t4758)
}
