//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 961/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk961(t9538: f64, t329: f64, t64: f64, t358: f64, t283: f64, t1135: f64, t9528: f64, t2817: f64, t2861: f64, t2822: f64, t2857: f64, t1018: f64, t86: f64, t9526: f64, sigma0: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t9539 = t9538 * sigma0;
    let t9543 = t64 * t329;
    let t9545 = 1.0_f64 / t358 / t9543;
    let t9546 = t283 * t9545;
    let t9552 = t9528 * t1135;
    let t9557 = t2861 * t2817;
    let t9559 = t2822 * t2857;
    let t9562 = t86 * t9526 * t1018;
    (t9539, t9545, t9546, t9552, t9557, t9559, t9562)
}
