//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1095/1225 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1095(t14365: f64, t1940: f64, t198: f64, t207: f64, t2071: f64, t2394: f64, t2403: f64, t2408: f64, t2430: f64, t26580: f64, t26585: f64, t26590: f64, t2832: f64, t4541: f64, t7428: f64, t7432: f64, t775: f64, t890: f64, t892: f64) -> f64 {
    let t26625 = t198 * t207 * t26580 * t892 - 6.0_f64 * t14365 * t2403 * t7432 + 2.0_f64 * t1940 * t2408 * t26590 - 2.0_f64 * t1940 * t26585 * t890 - t1940 * t2832 * t7432 + 6.0_f64 * t2071 * t2394 * t4541 + 3.0_f64 * t2071 * t2403 * t2430 + 6.0_f64 * t2403 * t7428 * t775;
    t26625
}
