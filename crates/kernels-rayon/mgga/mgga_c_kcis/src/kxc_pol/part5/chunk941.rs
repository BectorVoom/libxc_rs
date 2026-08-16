//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 941/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk941(t9112: f64, t9147: f64, t137: f64, t154: f64, t2588: f64, t2600: f64, t161: f64, t8538: f64, t8537: f64, t8750: f64, t755: f64, t159: f64, t689: f64) -> (f64, f64, f64, f64, f64) {
    let t9148 = t9112 + t9147;
    let t9149 = t9148 * t137;
    let t9150 = t9149 * t154;
    let t9152 = t2588 * t2600;
    let t9154 = t161 * t8538;
    let t9155 = t8537 * t9154;
    let t9157 = t161 * t8750;
    let t9158 = t755 * t9157;
    let t9160 = t159 * t689;
    (t9150, t9152, t9155, t9158, t9160)
}
