//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 793/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk793(t51: f64, t1368: f64, t35: f64, t1216: f64, t419: f64, t1225: f64, t1228: f64, t2474: f64, t2477: f64, t40: f64, t53: f64, t6991: f64, t6990: f64, zeta_threshold: f64) -> (f64, f64) {
    let t52 = t51 <= zeta_threshold;
    let t6994 = t1368 * t35;
    let t6995 = t1216 * t419;
    let t7005 = piecewise3(t52, 0.0_f64, -8.0_f64 / 27.0_f64 * t6991 * t1225 - 16.0_f64 / 9.0_f64 * t6994 * t6995 + 4.0_f64 / 9.0_f64 * t2474 * t1228 - 8.0_f64 / 3.0_f64 * t53 * t1216 + 8.0_f64 * t2477 * t40);
    let t7006 = t6990 + t7005;
    (t6995, t7006)
}
