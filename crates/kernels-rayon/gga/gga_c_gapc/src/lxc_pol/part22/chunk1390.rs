//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 1390/1426 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk1390(t34144: f64, t34146: f64, t34148: f64, t34154: f64, t34156: f64, t34161: f64, t34164: f64, t34169: f64, t34171: f64, t34174: f64, t34181: f64, t34184: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t36839 = 0.20240885416666666668e-4_f64 * t34144;
    let t36840 = 0.20240885416666666668e-3_f64 * t34146;
    let t36841 = 0.40481770833333333336e-4_f64 * t34148;
    let t36843 = 0.40481770833333333336e-4_f64 * t34154;
    let t36844 = 0.20240885416666666668e-4_f64 * t34156;
    let t36845 = 0.38647271295071362317e-7_f64 * t34161;
    let t36846 = 0.74216579861111111116e-4_f64 * t34164;
    let t36849 = 0.21135226489492151266e-6_f64 * t34169;
    let t36850 = 0.67528199161846004232e-6_f64 * t34171;
    let t36851 = 0.13505639832369200846e-5_f64 * t34174;
    let t36854 = 0.4637672555408563478e-4_f64 * t34181;
    let t36855 = 0.43440462632258606772e-4_f64 * t34184;
    (t36839, t36840, t36841, t36843, t36844, t36845, t36846, t36849, t36850, t36851, t36854, t36855)
}
