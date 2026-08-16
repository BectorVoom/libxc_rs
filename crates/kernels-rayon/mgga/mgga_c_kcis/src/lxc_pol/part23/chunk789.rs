//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 789/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk789(t12127: f64, t493: f64, t110: f64, t1381: f64, t109: f64, t1369: f64, t1372: f64, t1368: f64, t3970: f64, t3994: f64, t24: f64, t3977: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12129 = 5.0_f64 / 1296.0_f64 * t493 * t12127;
    let t12130 = t110 * t1381;
    let t12131 = t493 * t12130;
    let t12133 = t109 * t1369;
    let t12134 = t12133 * t1372;
    let t12135 = t1368 * t12134;
    let t12137 = t3970 * t3994;
    let t12138 = t1368 * t12137;
    let t12140 = t24 * t3977;
    (t12129, t12131, t12133, t12135, t12138, t12140)
}
