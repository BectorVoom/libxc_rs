//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1294/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1294(t65643: f64, t1844: f64, t30367: f64, t42181: f64, t5784: f64, t10292: f64, t18669: f64, t5489: f64, t6077: f64, t62280: f64, t18670: f64, t19404: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t67185 = 7.0_f64 / 144.0_f64 * t65643;
    let t67246 = t1844 * t30367;
    let t67326 = t42181 * t5784;
    let t67329 = t10292 * t18669;
    let t67331 = 80.0_f64 / 9.0_f64 * t67329 * t5489;
    let t67333 = 80.0_f64 / 9.0_f64 * t62280 * t6077;
    let t67335 = 80.0_f64 / 9.0_f64 * t18670 * t19404;
    (t67185, t67246, t67326, t67329, t67331, t67333, t67335)
}
