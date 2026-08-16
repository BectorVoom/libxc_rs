//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 39 (v4rho3tau_3) CSE chunk 1284/1328 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part39_v4rho3tau_3_chunk1284(t120: f64, t5187: f64, t1352: f64, t3805: f64, t3851: f64, t5301: f64, t1810: f64, t210: f64, t3734: f64, t3856: f64, t3793: f64, t5248: f64, t5249: f64) -> (f64, f64, f64, f64, f64) {
    let t16364 = t120 * t5187;
    let t16366 = t3805 * t16364 * t1352;
    let t16370 = t3805 * t5301 * t3851;
    let t16379 = t210 * t1810 * t3734;
    let t16383 = t3805 * t5301 * t3856;
    let t16387 = t5248 * t5249 * t3793;
    (t16366, t16370, t16379, t16383, t16387)
}
