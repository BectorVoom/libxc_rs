//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 673/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk673(t3198: f64, t1173: f64, t1184: f64, t1268: f64) -> (f64, f64, f64, f64) {
    let t3199 = 2.0_f64 * t3198;
    let t3200 = t1173 * t1184;
    let t3201 = 8.0_f64 * t3200;
    let t3202 = t1268 * t1268;
    (t3199, t3200, t3201, t3202)
}
