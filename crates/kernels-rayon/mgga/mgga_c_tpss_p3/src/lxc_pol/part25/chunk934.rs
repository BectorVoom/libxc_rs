//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 934/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk934(t3759: f64, t664: f64, t3803: f64, t673: f64, t1421: f64, t2202: f64, t3750: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t10982 = t664 * t3759;
    let t10983 = 0.19931111111111111111e0_f64 * t10982;
    let t10989 = t673 * t3803;
    let t10990 = 0.10954222222222222222e0_f64 * t10989;
    let t10994 = t2202 * t1421;
    let t11002 = t664 * t3750;
    (t10982, t10983, t10989, t10990, t10994, t11002)
}
