//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1077/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1077(t1811: f64, t22797: f64, t22804: f64, t7709: f64, t1361: f64, t1799: f64, t22690: f64, t22792: f64, t5227: f64, t6916: f64, t1998: f64, t236: f64, t5187: f64) -> (f64, f64, f64, f64, f64) {
    let t26266 = t22797 * t1811;
    let t26268 = t22804 * t7709;
    let t26271 = t22690 * t1361 * t1799;
    let t26272 = t22792 * t26271;
    let t26274 = t6916 * t5227;
    let t26277 = t1998 * t236 * t5187;
    (t26266, t26268, t26272, t26274, t26277)
}
