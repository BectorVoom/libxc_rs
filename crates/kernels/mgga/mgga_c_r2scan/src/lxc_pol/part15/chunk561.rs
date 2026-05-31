//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 561/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk561<F: Float>(t44: F, t51: F, t2526: F, t506: F, t529: F, t35: F, t99: F, t1216: F, t415: F, t903: F, t101: F, t419: F, t906: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t45 = t44 <= zeta_threshold;
    let t52 = t51 <= zeta_threshold;
    let t2698 = t506 * t2526;
    let t2699 = t529 * t2698;
    let t2706 = t99 * t35;
    let t2710 = piecewise3::<F>(t45, F::cast_from(0.0_f64), F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t903 * t415 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t2706 * t1216);
    let t2713 = t101 * t35;
    let t2717 = piecewise3::<F>(t52, F::cast_from(0.0_f64), F::cast_from(10.0_f64) / F::cast_from(9.0_f64) * t906 * t419 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t2713 * t1216);
    let t2719 = t2710 / F::cast_from(2.0_f64) + t2717 / F::cast_from(2.0_f64);
    (t2698, t2699, t2706, t2713, t2719)
}
