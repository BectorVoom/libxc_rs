//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1371/1395 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1371(t2061: f64, t6308: f64, t42181: f64, t5784: f64, t10292: f64, t18669: f64, t5489: f64, t6077: f64, t62280: f64, t18670: f64, t19404: f64, t19408: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t67316 = t6308 * t2061;
    let t67326 = t42181 * t5784;
    let t67329 = t10292 * t18669;
    let t67331 = 80.0_f64 / 9.0_f64 * t67329 * t5489;
    let t67333 = 80.0_f64 / 9.0_f64 * t62280 * t6077;
    let t67335 = 80.0_f64 / 9.0_f64 * t18670 * t19404;
    let t67337 = 80.0_f64 / 9.0_f64 * t18670 * t19408;
    (t67316, t67326, t67331, t67333, t67335, t67337)
}
