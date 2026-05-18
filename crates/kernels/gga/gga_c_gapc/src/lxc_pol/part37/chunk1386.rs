//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 1386/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk1386<F: Float>(t34148: F, t34154: F, t34156: F, t34161: F, t34164: F, t34169: F, t34171: F, t34174: F, t34181: F, t34184: F, t34188: F, t34191: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t36841 = F::new(0.40481770833333333336e-4) * t34148;
    let t36843 = F::new(0.40481770833333333336e-4) * t34154;
    let t36844 = F::new(0.20240885416666666668e-4) * t34156;
    let t36845 = F::new(0.38647271295071362317e-7) * t34161;
    let t36846 = F::new(0.74216579861111111116e-4) * t34164;
    let t36849 = F::new(0.21135226489492151266e-6) * t34169;
    let t36850 = F::new(0.67528199161846004232e-6) * t34171;
    let t36851 = F::new(0.13505639832369200846e-5) * t34174;
    let t36854 = F::new(0.4637672555408563478e-4) * t34181;
    let t36855 = F::new(0.43440462632258606772e-4) * t34184;
    let t36856 = F::new(0.3437982149563945044e-8) * t34188;
    let t36857 = F::new(0.2845640240200497334e-7) * t34191;
    (t36841, t36843, t36844, t36845, t36846, t36849, t36850, t36851, t36854, t36855, t36856, t36857)
}
