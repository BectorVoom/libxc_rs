//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1016/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1016(t1020: f64, t3388: f64, t1083: f64, t1085: f64, t1087: f64, t1089: f64, t11930: f64, t2410: f64, t3390: f64, t3394: f64, t3398: f64, t3402: f64, t3406: f64, t3652: f64, t3656: f64, t3660: f64, t3664: f64, t839: f64) -> f64 {
    let t11932 = t3388 * t1020;
    let t11960 = -0.64e0_f64 * t11930 - 0.9214113627294e1_f64 * t11932 - 0.9214113627294e1_f64 * t3390 * t1020 - 0.9214113627294e1_f64 * t1083 * t2410 - 0.9214113627294e1_f64 * t3652 * t839 + 0.367387230261e2_f64 * t3394 * t1020 + 0.367387230261e2_f64 * t1085 * t2410 + 0.367387230261e2_f64 * t3656 * t839 - 0.3831420472412e2_f64 * t3398 * t1020 - 0.3831420472412e2_f64 * t1087 * t2410 - 0.3831420472412e2_f64 * t3660 * t839 + 0.1550653405116e2_f64 * t3402 * t1020 + 0.1550653405116e2_f64 * t1089 * t2410 + 0.1550653405116e2_f64 * t3664 * t839 - 0.2177652951264e1_f64 * t3406 * t1020;
    t11960
}
