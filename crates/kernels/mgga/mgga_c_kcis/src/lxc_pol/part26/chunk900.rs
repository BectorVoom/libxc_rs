//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 900/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk900<F: Float>(t21134: F, t3883: F, t4714: F, t1330: F, t21106: F, t26: F, t21110: F, t21073: F, t21196: F, t21199: F, t21201: F, t21203: F, t21206: F, t21209: F, t21212: F) -> (F, F, F, F, F) {
    let t21214 = t3883 * t21134;
    let t21215 = t4714 * t21214;
    let t21217 = t1330 * t21106;
    let t21218 = t26 * t21217;
    let t21220 = t1330 * t21110;
    let t21221 = t4714 * t21220;
    let t21223 = t1330 * t21073;
    let t21224 = t26 * t21223;
    let t21226 = F::new(0.99655555555555555557e-1) * t21196 - F::new(0.82156666666666666667e-1) * t21199 - F::new(0.10954222222222222222e0) * t21201 + F::new(0.54771111111111111111e-1) * t21203 - F::new(0.23917333333333333334e1) * t21206 - F::new(0.19931111111111111111e0) * t21209 + F::new(0.59793333333333333334e0) * t21212 + F::new(0.10954222222222222222e0) * t21215 - F::new(0.49293999999999999999e0) * t21218 - F::new(0.65725333333333333332e0) * t21221 + F::new(0.16431333333333333333e0) * t21224;
    (t21215, t21218, t21221, t21224, t21226)
}
