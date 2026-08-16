//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 856/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk856(t1317: f64, t16194: f64, t16060: f64, t3883: f64, t26: f64, t1330: f64, t16073: f64, t16069: f64, t11462: f64, t16055: f64, t16065: f64, t4714: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16195 = t1317 * t16194;
    let t16197 = t3883 * t16060;
    let t16198 = t26 * t16197;
    let t16200 = t1330 * t16073;
    let t16201 = t26 * t16200;
    let t16203 = t3883 * t16069;
    let t16204 = t26 * t16203;
    let t16206 = t11462 * t16055;
    let t16207 = t26 * t16206;
    let t16209 = t3883 * t16065;
    let t16210 = t4714 * t16209;
    (t16195, t16198, t16201, t16204, t16207, t16210)
}
