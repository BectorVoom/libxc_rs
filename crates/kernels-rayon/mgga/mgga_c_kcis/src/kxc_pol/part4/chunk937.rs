//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 937/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk937(t2605: f64, t823: f64, t2489: f64, t804: f64, t2594: f64, t158: f64, t2490: f64, t160: f64, t774: f64, t2526: f64, t2612: f64, t8531: f64) -> (f64, f64, f64, f64) {
    let t9040 = t2605 * t823;
    let t9042 = t804 * t2489;
    let t9043 = t9042 * t2594;
    let t9045 = t2490 * t158;
    let t9046 = t160 * t774;
    let t9047 = t9046 * t2526;
    let t9048 = t9045 * t9047;
    let t9050 = t8531 * t2612;
    (t9040, t9043, t9048, t9050)
}
