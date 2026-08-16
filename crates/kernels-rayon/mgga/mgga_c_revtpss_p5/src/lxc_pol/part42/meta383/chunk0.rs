//! MGGA_C_REVTPSS lxc pol — lxc_pol part 42 (v4rho3tau_5) CSE chunk 1265/1505 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part42_v4rho3tau_5_chunk1265(t1079: f64, t1651: f64, t5015: f64, t4772: f64, t996: f64, t16313: f64, t4940: f64, t6258: f64, t999: f64, t1096: f64, t6244: f64, t6350: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t19396 = t1079 * t1651 * t5015;
    let t19399 = t1651 * t4772;
    let t19400 = t996 * t19399;
    let t19403 = t16313 * t4940;
    let t19414 = t6258 * t999;
    let t19415 = t996 * t19414;
    let t19421 = t1079 * t6244 * t1096;
    let t19424 = t6350 * t1096;
    (t19396, t19399, t19400, t19403, t19414, t19415, t19421, t19424)
}
