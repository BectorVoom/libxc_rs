//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 139/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk139(t411: f64, t415: f64, t135: f64, t273: f64, t354: f64, t382: f64, t384: f64) -> (f64, f64) {
    let t418 = 1.0_f64 + 0.65854491829355115987e0_f64 * t411 * t415;
    let t419 = f64::ln(t418);
    let t422 = t135 * t273 * t419 - t354 + t382 + t384;
    (t418, t422)
}
