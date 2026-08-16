//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 931/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk931(t2334: f64, t3572: f64, t1289: f64, t2332: f64, t681: f64, t1351: f64, t37: f64, t177: f64, t3590: f64, t737: f64, t162: f64, t8087: f64) -> (f64, f64, f64, f64, f64) {
    let t10706 = 8.0_f64 * t3572 * t2334;
    let t10707 = t2332 * t1289;
    let t10708 = t681 * t10707;
    let t10710 = t37 * t1351;
    let t10717 = t3590 * t177;
    let t10719 = 0.11696447245269292414e1_f64 * t10717 * t737;
    let t10728 = t8087 * t162;
    (t10706, t10708, t10710, t10719, t10728)
}
