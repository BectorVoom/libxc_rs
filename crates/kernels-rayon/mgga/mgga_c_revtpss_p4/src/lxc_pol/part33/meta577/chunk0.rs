//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1986/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1986(t5876: f64, t670: f64, t1448: f64, t6836: f64, t6816: f64, t1868: f64, t5778: f64, t10309: f64, t607: f64, t843: f64, t1962: f64, t41154: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t85360 = t5876 * t670;
    let t86753 = t6836 * t1448;
    let t86771 = t6816 * t1448;
    let t86815 = t1868 * t5778;
    let t92568 = t10309 * t607;
    let t92612 = 1232.0_f64 / 27.0_f64 * t843;
    let t92742 = t1962 * t41154;
    (t85360, t86753, t86771, t86815, t92568, t92612, t92742)
}
