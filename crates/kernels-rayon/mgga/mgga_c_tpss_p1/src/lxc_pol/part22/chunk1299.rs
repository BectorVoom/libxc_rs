//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1299/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1299(t1675: f64, t18331: f64, t5790: f64, t18646: f64, t5483: f64, t18645: f64, t5506: f64, t18305: f64, t5791: f64, t18660: f64, t18360: f64, t18670: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t62250 = t1675 * t5790 * t18331;
    let t62259 = t5483 * t18646;
    let t62262 = t1675 * t18645 * t5506;
    let t62264 = t18305 * t5791;
    let t62266 = t5483 * t18660;
    let t62270 = t18670 * t18360;
    (t62250, t62259, t62262, t62264, t62266, t62270)
}
