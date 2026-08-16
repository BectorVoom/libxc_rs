//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1879/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1879(t2453: f64, t2458: f64, t7399: f64, t2070: f64, t41154: f64, t11064: f64, t7427: f64, t25876: f64, t26304: f64, t25894: f64, t94398: f64, t122: f64, t72: f64, t7506: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t95948 = t2453 * t7399 * t2458;
    let t95964 = t2070 * t41154;
    let t95976 = t7427 * t11064;
    let t96186 = t25876 * t26304;
    let t96187 = t25894 * t96186;
    let t96188 = t96187 * t94398;
    let t96191 = t7506 * t72 * t122;
    (t95948, t95964, t95976, t96186, t96187, t96188, t96191)
}
