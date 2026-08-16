//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1916/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1916(t22986: f64, t23270: f64, t25191: f64, t4300: f64, t25192: f64, t86873: f64, t5544: f64, t857: f64, t865: f64, t1527: f64, t86849: f64, t4272: f64, t86969: f64) -> (f64, f64, f64, f64, f64) {
    let t98248 = t22986 * t23270 * t25191 * t4300;
    let t98251 = t22986 * t86873 * t25192;
    let t98253 = t857 * t5544;
    let t98256 = t22986 * t23270 * t98253 * t865;
    let t98264 = t22986 * t23270 * t86849 * t1527;
    let t98277 = t22986 * t23270 * t86969 * t4272;
    (t98248, t98251, t98256, t98264, t98277)
}
