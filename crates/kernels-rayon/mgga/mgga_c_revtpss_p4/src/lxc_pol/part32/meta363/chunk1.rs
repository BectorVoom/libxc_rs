//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1312/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1312(t15127: f64, t4625: f64, t698: f64, t4622: f64, t1593: f64, t2435: f64) -> (f64, f64, f64, f64, f64) {
    let t15128 = 0.13418888888888888889e0_f64 * t15127;
    let t15168 = t698 * t4625;
    let t15169 = 0.22076e0_f64 * t15168;
    let t15170 = t698 * t4622;
    let t15189 = t2435 * t1593;
    (t15128, t15168, t15169, t15170, t15189)
}
