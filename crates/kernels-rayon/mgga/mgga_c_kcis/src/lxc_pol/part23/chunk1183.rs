//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1183/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1183(t16968: f64, t3717: f64, t1380: f64, t1385: f64, t1377: f64, t1593: f64, t27371: f64, t27369: f64, t52613: f64, t7908: f64, t7910: f64, t27376: f64, t27459: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t94228 = t16968 * t3717;
    let t94229 = t1380 * t1385;
    let t94246 = t1593 * t1377;
    let t94247 = t94246 * t27371;
    let t94248 = t27369 * t94247;
    let t94287 = t7908 * t52613 * t7910;
    let t94289 = t27459 * t27376;
    (t94228, t94229, t94246, t94247, t94248, t94287, t94289)
}
