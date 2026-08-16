//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1403/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1403(t20497: f64, t22761: f64, t20512: f64, t80830: f64, t1998: f64, t20416: f64, t236: f64, t6926: f64, t20470: f64, t26309: f64, t26257: f64, t6431: f64) -> (f64, f64, f64, f64, f64) {
    let t107093 = t22761 * t20497;
    let t107096 = t80830 * t20512;
    let t107100 = t6926 * t1998 * t236 * t20416;
    let t107102 = t26309 * t20470;
    let t107105 = t26257 * t6431;
    (t107093, t107096, t107100, t107102, t107105)
}
