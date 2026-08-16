//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 960/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk960(t225: f64, t6005: f64, t2638: f64, t5966: f64, t5962: f64, t832: f64, t1553: f64, t1555: f64, t227: f64, t229: f64) -> (f64, f64, f64, f64) {
    let t6006 = t6005 * t225;
    let t6010 = t2638 * t5966;
    let t6013 = t832 * t5962;
    let t6016 = 6.0_f64 * t1553 * t1555 - 12.0_f64 * t227 * t6010 + 3.0_f64 * t227 * t6013 - t229 * t6006;
    (t6006, t6010, t6013, t6016)
}
