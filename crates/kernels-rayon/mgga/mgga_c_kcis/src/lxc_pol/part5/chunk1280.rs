//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1280/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1280(t21134: f64, t3883: f64, t4714: f64, t1330: f64, t21106: f64, t26: f64, t21110: f64, t21073: f64, t21196: f64, t21199: f64, t21201: f64, t21203: f64, t21206: f64, t21209: f64, t21212: f64) -> (f64, f64, f64, f64, f64) {
    let t21214 = t3883 * t21134;
    let t21215 = t4714 * t21214;
    let t21217 = t1330 * t21106;
    let t21218 = t26 * t21217;
    let t21220 = t1330 * t21110;
    let t21221 = t4714 * t21220;
    let t21223 = t1330 * t21073;
    let t21224 = t26 * t21223;
    let t21226 = 0.99655555555555555557e-1_f64 * t21196 - 0.82156666666666666667e-1_f64 * t21199 - 0.10954222222222222222e0_f64 * t21201 + 0.54771111111111111111e-1_f64 * t21203 - 0.23917333333333333334e1_f64 * t21206 - 0.19931111111111111111e0_f64 * t21209 + 0.59793333333333333334e0_f64 * t21212 + 0.10954222222222222222e0_f64 * t21215 - 0.49293999999999999999e0_f64 * t21218 - 0.65725333333333333332e0_f64 * t21221 + 0.16431333333333333333e0_f64 * t21224;
    (t21215, t21218, t21221, t21224, t21226)
}
