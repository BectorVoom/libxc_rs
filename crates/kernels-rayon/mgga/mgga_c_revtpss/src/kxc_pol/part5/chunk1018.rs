//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1018/1422 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1018(t11670: f64, t3089: f64, t1087: f64, t3090: f64, t3278: f64, t3182: f64, t828: f64, t3109: f64, t126: f64, t3181: f64, t1003: f64, t3080: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t11671 = t11670 * t3089;
    let t11672 = t1087 * t11671;
    let t11675 = t3278 * t3090;
    let t11703 = t828 * t3182;
    let t11710 = t828 * t3109;
    let t11725 = t126 * t3181;
    let t11732 = t1003 * t3080;
    (t11671, t11672, t11675, t11703, t11710, t11725, t11732)
}
