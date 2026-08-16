//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1176/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1176(t215: f64, t39933: f64, t535: f64, t12227: f64, t9577: f64, t116: f64, t557: f64, t212: f64, t2586: f64, t3734: f64, t12225: f64, t3719: f64) -> (f64, f64, f64, f64) {
    let t40350 = 0.14979423868312757201e0_f64 * t39933 * t535 * t215;
    let t40351 = t9577 * t12227;
    let t40353 = t557 * t116;
    let t40356 = t2586 * t40353 * t212 * t3734;
    let t40360 = t2586 * t12225 * t212 * t3719;
    (t40350, t40351, t40356, t40360)
}
