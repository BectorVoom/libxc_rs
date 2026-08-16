//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1159/1341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1159(t1665: f64, t1671: f64, t25500: f64, t25505: f64, t25509: f64, t25517: f64, t25522: f64, t25560: f64, t25580: f64, t27450: f64, t27479: f64, t27539: f64, t6263: f64, t6268: f64, t6273: f64, t6278: f64, t6302: f64, t6308: f64, t6312: f64, t6331: f64, t6339: f64, t7117: f64, t7122: f64, t7132: f64) -> f64 {
    let t29806 = -0.57165357490759649296e-3_f64 * t7132 * t6331 + 0.57165357490759649296e-3_f64 * t25517 * t6268 + 0.42874018118069736972e-3_f64 * t7122 * t6302 + 0.85748036236139473944e-3_f64 * t25505 * t6308 - 0.42874018118069736972e-3_f64 * t25509 * t6312 + 0.85748036236139473944e-3_f64 * t27450 * t1671 - 0.57165357490759649296e-3_f64 * t25522 * t6263 + 0.85748036236139473944e-3_f64 * t25500 * t6339 - 0.85748036236139473944e-3_f64 * t27479 * t1665 - 0.42874018118069736972e-3_f64 * t7117 * t6278 - t25560 + 0.3811023832717309953e-3_f64 * t27539 - 0.85748036236139473944e-3_f64 * t25580 * t6273;
    t29806
}
