//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1393/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1393<F: Float>(t34144: F, t34146: F, t34148: F, t34154: F, t34156: F, t34161: F, t34164: F, t34169: F, t34171: F, t34174: F, t34181: F, t34184: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36839 = F::cast_from(0.20240885416666666668e-4_f64) * t34144;
    let t36840 = F::cast_from(0.20240885416666666668e-3_f64) * t34146;
    let t36841 = F::cast_from(0.40481770833333333336e-4_f64) * t34148;
    let t36843 = F::cast_from(0.40481770833333333336e-4_f64) * t34154;
    let t36844 = F::cast_from(0.20240885416666666668e-4_f64) * t34156;
    let t36845 = F::cast_from(0.38647271295071362317e-7_f64) * t34161;
    let t36846 = F::cast_from(0.74216579861111111116e-4_f64) * t34164;
    let t36849 = F::cast_from(0.21135226489492151266e-6_f64) * t34169;
    let t36850 = F::cast_from(0.67528199161846004232e-6_f64) * t34171;
    let t36851 = F::cast_from(0.13505639832369200846e-5_f64) * t34174;
    let t36854 = F::cast_from(0.4637672555408563478e-4_f64) * t34181;
    let t36855 = F::cast_from(0.43440462632258606772e-4_f64) * t34184;
    (t36839, t36840, t36841, t36843, t36844, t36845, t36846, t36849, t36850, t36851, t36854, t36855)
}
