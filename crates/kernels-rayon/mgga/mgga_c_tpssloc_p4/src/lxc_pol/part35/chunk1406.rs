//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 35 (v4rho3sigma_11) CSE chunk 1406/1466 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part35_v4rho3sigma_11_chunk1406(t20479: f64, t6952: f64, t1831: f64, t97265: f64, t1998: f64, t20356: f64, t236: f64, t80894: f64, t1799: f64, t22827: f64, t3788: f64, t6388: f64) -> (f64, f64, f64, f64) {
    let t107133 = t6952 * t20479;
    let t107135 = t97265 * t1831;
    let t107139 = t80894 * t1998 * t236 * t20356;
    let t107143 = t22827 * t3788 * t6388 * t1799;
    (t107133, t107135, t107139, t107143)
}
