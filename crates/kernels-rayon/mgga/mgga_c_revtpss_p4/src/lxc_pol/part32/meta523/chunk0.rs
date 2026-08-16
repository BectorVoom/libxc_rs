//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1826/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1826(t5876: f64, t670: f64, t1448: f64, t6836: f64, t6816: f64, t1868: f64, t5778: f64, t10309: f64, t607: f64, t2411: f64, t605: f64, t1955: f64, t25308: f64, t2769: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t85360 = t5876 * t670;
    let t86753 = t6836 * t1448;
    let t86771 = t6816 * t1448;
    let t86815 = t1868 * t5778;
    let t92568 = t10309 * t607;
    let t92790 = t2411 * t605;
    let t92917 = t1955 * t25308 * t2769;
    (t85360, t86753, t86771, t86815, t92568, t92790, t92917)
}
