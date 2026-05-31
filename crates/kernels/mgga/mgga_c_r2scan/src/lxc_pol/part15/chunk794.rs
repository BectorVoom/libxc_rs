//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 794/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk794<F: Float>(t51: F, t1368: F, t35: F, t1216: F, t419: F, t1225: F, t1228: F, t2474: F, t2477: F, t40: F, t53: F, t6991: F, t6990: F, zeta_threshold: F) -> (F, F) {
    let t52 = t51 <= zeta_threshold;
    let t6994 = t1368 * t35;
    let t6995 = t1216 * t419;
    let t7005 = piecewise3::<F>(t52, F::cast_from(0.0_f64), -F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t6991 * t1225 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t6994 * t6995 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t2474 * t1228 - F::cast_from(8.0_f64) / F::cast_from(3.0_f64) * t53 * t1216 + F::cast_from(8.0_f64) * t2477 * t40);
    let t7006 = t6990 + t7005;
    (t6995, t7006)
}
