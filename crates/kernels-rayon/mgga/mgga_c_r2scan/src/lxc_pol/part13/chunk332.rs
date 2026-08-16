//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 332/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk332(t44: f64, t1214: f64, t1219: f64, t472: f64, t54: f64, t419: f64, zeta_threshold: f64) -> (f64, f64, f64) {
    let t45 = t44 <= zeta_threshold;
    let t1223 = piecewise3(t45, 0.0_f64, -2.0_f64 / 9.0_f64 * t1214 + 2.0_f64 / 3.0_f64 * t472 * t1219);
    let t1224 = 1.0_f64 / t54;
    let t1225 = t419 * t419;
    (t1223, t1224, t1225)
}
