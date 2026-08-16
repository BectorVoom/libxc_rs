//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1338/1475 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1338(t31170: f64, t5259: f64, t5293: f64, t5303: f64, t114016: f64, t5252: f64, t1998: f64, t5187: f64, t59: f64, t6926: f64, t5287: f64, t6936: f64, t6943: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t120395 = t31170 * t5259;
    let t120397 = t31170 * t5293;
    let t120399 = t31170 * t5303;
    let t120401 = t114016 * t5252;
    let t120405 = t6926 * t1998 * t59 * t5187;
    let t120408 = t6936 * t6943 * t5287;
    (t120395, t120397, t120399, t120401, t120405, t120408)
}
