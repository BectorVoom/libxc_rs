//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 1946/2275 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1946(t1963: f64, t5966: f64, t1544: f64, t1583: f64, t1940: f64, t198: f64, t207: f64, t2403: f64, t25445: f64, t27368: f64, t29598: f64, t29704: f64, t4541: f64, t5962: f64, t6075: f64, t6079: f64, t7091: f64, t7783: f64, t892: f64) -> (f64, f64) {
    let t29907 = t1963 * t5966;
    let t29930 = t198 * t207 * t29704 * t892 + 6.0_f64 * t1544 * t2403 * t7783 - 2.0_f64 * t1583 * t1940 * t27368 + 2.0_f64 * t1940 * t25445 * t6079 - t1940 * t6075 * t7091 + 3.0_f64 * t1963 * t2403 * t5962 - 6.0_f64 * t2403 * t29598 * t7091 + 6.0_f64 * t29907 * t4541;
    (t29907, t29930)
}
