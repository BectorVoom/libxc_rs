//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1114/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1114(t330: f64, t6320: f64, t10324: f64, t829: f64, t1670: f64, t1727: f64, t14196: f64, t934: f64, t1045: f64, t14170: f64, t347: f64, t6338: f64) -> (f64, f64, f64, f64) {
    let t18773 = t6320 * t330;
    let t18775 = t10324 * t18773 * t829;
    let t18778 = t1670 * t1727;
    let t18780 = t14196 * t18778 * t934;
    let t18784 = t14170 * t18778 * t1045;
    let t18787 = t347 * t6338;
    (t18775, t18780, t18784, t18787)
}
