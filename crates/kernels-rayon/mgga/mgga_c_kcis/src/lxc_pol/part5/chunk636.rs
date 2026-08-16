//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 636/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk636(t4413: f64, t609: f64, t109: f64, t494: f64, t209: f64, t617: f64, t612: f64, t1369: f64, t25: f64, t1602: f64, t1599: f64, t1611: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t4414 = t609 * t4413;
    let t4420 = t109 * t494;
    let t4422 = t209 * t4420 * t617;
    let t4424 = t612 * t4422 / 864.0_f64;
    let t4425 = t25 * t1369;
    let t4426 = t4425 * t1602;
    let t4427 = t1599 * t4426;
    let t4429 = t25 * t1611;
    (t4414, t4422, t4424, t4425, t4426, t4427, t4429)
}
