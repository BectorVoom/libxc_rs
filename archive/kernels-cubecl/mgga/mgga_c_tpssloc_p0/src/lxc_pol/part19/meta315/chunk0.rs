//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1121/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1121<F: Float>(t28: F, t3673: F, t3231: F, t39109: F, t11122: F, t12072: F, t12075: F, t3672: F, t39436: F, t517: F, t157: F, t39434: F, t182: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t29 = t28 <= zeta_threshold;
    let t39437 = t3673 * t3673;
    let t39443 = t3231 * t3231;
    let t39448 = -t39109;
    let t39452 = piecewise3::<F>(t29, F::cast_from(0.0_f64), F::cast_from(40.0_f64) / F::cast_from(81.0_f64) * t39436 * t39437 - F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t12072 * t3673 * t3231 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t3672 * t39443 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t12075 * t11122 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t517 * t39448);
    let t39454 = (t39434 + t39452) * t157;
    let t39456 = F::cast_from(0.19751673498613801407e-1_f64) * t39454 * t182;
    (t39437, t39443, t39448, t39454, t39456)
}
