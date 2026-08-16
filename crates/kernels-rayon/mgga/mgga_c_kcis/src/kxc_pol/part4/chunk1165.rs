//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1165/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1165(t14092: f64, t5176: f64, t5175: f64, t13260: f64, t5077: f64, t3337: f64, t10707: f64, t5091: f64, t1797: f64, t3365: f64, t1816: f64, t3354: f64) -> (f64, f64, f64, f64, f64) {
    let t14765 = t5176 * t14092;
    let t14766 = t5175 * t14765;
    let t14768 = t5077 * t13260;
    let t14769 = t3337 * t14768;
    let t14771 = t10707 * t5091;
    let t14773 = t1797 * t3365;
    let t14775 = t3354 * t1816;
    (t14766, t14769, t14771, t14773, t14775)
}
