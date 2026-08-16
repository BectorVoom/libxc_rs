//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1236/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1236(t12651: f64, t1616: f64, t1370: f64, t27614: f64, t94588: f64, t1444: f64, t4314: f64, t27651: f64, t7964: f64, t2257: f64, t2259: f64, t44682: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t95009 = t12651 * t1616;
    let t95024 = t1370 * t27614;
    let t95088 = 0.51588271604938271604e-3_f64 * t94588;
    let t95103 = t4314 * t1444;
    let t95137 = t7964 * t27651;
    let t95168 = 0.12871334876543209877e-3_f64 * t2257 * t44682 * t2259;
    (t95009, t95024, t95088, t95103, t95137, t95168)
}
