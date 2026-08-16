//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1780/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1780(t111: f64, t7222: f64, t81437: f64, t22550: f64, t7031: f64, t39054: f64, t7025: f64, t23966: f64, t9231: f64, t39063: f64, t9239: f64, t1860: f64, t23992: f64, t6509: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t84033 = t7222 * t111;
    let t84036 = 308.0_f64 / 27.0_f64 * t81437;
    let t84173 = t7031 * t22550;
    let t84190 = t39054 * t7025;
    let t84195 = t9231 * t23966;
    let t84216 = t39063 * t7025;
    let t84219 = t9239 * t23966;
    let t84229 = t1860 * t23992 * t6509;
    (t84033, t84036, t84173, t84190, t84195, t84216, t84219, t84229)
}
