//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 1797/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk1797(t25249: f64, t776: f64, t25248: f64, t25038: f64, t7510: f64, t814: f64, t829: f64, t7528: f64, t794: f64, t6562: f64, t1509: f64, t1902: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t25250 = t25249 * t776;
    let t25251 = t25248 * t25250;
    let t25252 = t25038 * t25251;
    let t25255 = t814 * t7510;
    let t25256 = t25255 * t829;
    let t25258 = t794 * t7528;
    let t25259 = t6562 * t25258;
    let t25261 = t1902 * t1509;
    (t25250, t25251, t25252, t25255, t25256, t25258, t25259, t25261)
}
