//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1095/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1095(t2071: f64, t5966: f64, t1544: f64, t1583: f64, t1940: f64, t198: f64, t207: f64, t2403: f64, t26590: f64, t28460: f64, t29598: f64, t30419: f64, t4541: f64, t5962: f64, t6075: f64, t6079: f64, t7432: f64, t8020: f64, t892: f64) -> f64 {
    let t30439 = t2071 * t5966;
    let t30462 = t198 * t207 * t30419 * t892 + 6.0_f64 * t1544 * t2403 * t8020 - 2.0_f64 * t1583 * t1940 * t28460 + 2.0_f64 * t1940 * t26590 * t6079 - t1940 * t6075 * t7432 + 3.0_f64 * t2071 * t2403 * t5962 - 6.0_f64 * t2403 * t29598 * t7432 + 6.0_f64 * t30439 * t4541;
    t30462
}
