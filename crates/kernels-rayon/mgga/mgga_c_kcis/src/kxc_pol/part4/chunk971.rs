//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 971/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk971(t9725: f64, t2937: f64, t926: f64, t2997: f64, t45: f64, t270: f64, t3030: f64, t9728: f64, t999: f64, t292: f64, t737: f64, t285: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t9775 = 0.28842592592592592592e-1_f64 * t9725;
    let t9790 = 0.55403703703703703703e-1_f64 * t9725;
    let t9804 = t926 * t2937;
    let t9817 = t45 * t2997;
    let t9825 = 1.0_f64 / t3030 / t270;
    let t9851 = 0.93932222222222222223e0_f64 * t9725;
    let t9852 = 0.36793333333333333333e0_f64 * t9728;
    let t9873 = t999 * t999;
    let t9874 = 1.0_f64 / t9873;
    let t9881 = t737 * t292;
    let t9883 = 5.0_f64 / 1296.0_f64 * t285 * t9881;
    (t9775, t9790, t9804, t9817, t9825, t9851, t9852, t9874, t9883)
}
