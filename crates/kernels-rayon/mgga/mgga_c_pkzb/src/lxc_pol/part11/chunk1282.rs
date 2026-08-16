//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1282/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1282(t18427: f64, t18468: f64, t22230: f64, t22302: f64, t27262: f64, t27295: f64, t31067: f64, t31088: f64, t834: f64, t841: f64, t218: f64, t219: f64, t3026: f64, t3730: f64) -> (f64, f64, f64) {
    let t31239 = t18468 - 28.0_f64 / 27.0_f64 * t18427 - 28.0_f64 / 9.0_f64 * t22230 + t22302 + 4.0_f64 / 3.0_f64 * t27295 - t27262 - t31067 / 3.0_f64 + t31088;
    let t31240 = t834 * t31239;
    let t31242 = t841 * t31239;
    let t31250 = t218 * t219 * t3026 * t3730;
    (t31240, t31242, t31250)
}
