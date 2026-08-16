//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1162/1360 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1162(t25637: f64, t25704: f64, t3336: f64, t7177: f64, t11108: f64, t1989: f64, t14365: f64, t1940: f64, t1963: f64, t198: f64, t207: f64, t2394: f64, t2403: f64, t2408: f64, t2430: f64, t25435: f64, t25440: f64, t25445: f64, t2832: f64, t4541: f64, t7087: f64, t7091: f64, t775: f64, t890: f64, t892: f64) -> (f64, f64, f64, f64) {
    let t25705 = t25637 + t25704;
    let t25709 = t7177 * t3336;
    let t25713 = t1989 * t11108;
    let t25743 = t198 * t207 * t25435 * t892 - 6.0_f64 * t14365 * t2403 * t7091 + 2.0_f64 * t1940 * t2408 * t25445 - 2.0_f64 * t1940 * t25440 * t890 - t1940 * t2832 * t7091 + 6.0_f64 * t1963 * t2394 * t4541 + 3.0_f64 * t1963 * t2403 * t2430 + 6.0_f64 * t2403 * t7087 * t775;
    (t25705, t25709, t25713, t25743)
}
