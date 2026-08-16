//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 879/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk879(t1243: f64, t236: f64, t615: f64, t7230: f64, t7231: f64, t34847: f64, t8831: f64, t1550: f64, t5144: f64, t7778: f64, t2060: f64, t27177: f64, t4044: f64) -> (f64, f64, f64, f64) {
    let t39523 = t7230 * t7231 * t236 * t615 * t1243;
    let t39525 = t34847 * t8831;
    let t39528 = t1550 * t7778 * t5144;
    let t39531 = t4044 * t2060 * t27177;
    (t39523, t39525, t39528, t39531)
}
