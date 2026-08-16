//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1306/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1306(t1981: f64, t4580: f64, t4626: f64, t615: f64, t77: f64, t13447: f64, t84: f64, t3431: f64, t1976: f64, t13330: f64, t578: f64, t13336: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t69210 = t1981 * t4580;
    let t69228 = t77 * t615 * t4626;
    let t69232 = t77 * t84 * t13447;
    let t69242 = t77 * t84 * t3431;
    let t69245 = t1976 * t4580;
    let t69248 = t578 * t13330;
    let t69251 = t578 * t13336;
    (t69210, t69228, t69232, t69242, t69245, t69248, t69251)
}
