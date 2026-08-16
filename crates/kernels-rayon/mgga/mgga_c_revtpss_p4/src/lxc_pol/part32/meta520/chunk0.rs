//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1823/2056 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1823(t14586: f64, t4423: f64, t231: f64, t61749: f64, t61756: f64, t1544: f64, t2411: f64, t22461: f64, t4147: f64, t6861: f64, t9994: f64, t1398: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t62628 = t14586 * t4423;
    let t62637 = t61749 * t231;
    let t62695 = t61756 * t231;
    let t63185 = t2411 * t1544;
    let t73407 = t22461 * t4147;
    let t73820 = t6861 * t9994;
    let t73842 = t6861 * t1398;
    (t62628, t62637, t62695, t63185, t73407, t73820, t73842)
}
