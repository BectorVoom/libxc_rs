//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3174/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3174<F: Float>(t11789: F, t1227: F, t248: F, t5975: F, t18321: F, t3548: F, t15437: F, t15502: F, t15506: F, t4965: F, t5023: F, t1232: F, t15498: F, t15594: F, t19083: F, t3511: F, t3518: F, t3527: F, t3531: F, t44811: F, t4974: F, t52575: F, t52580: F, t52583: F, t52586: F, t52599: F) -> F {
    let t65689 = t1227 * t248 * t11789 * t5975;
    let t65691 = t18321 * t3548;
    let t65703 = t15437 * t15502;
    let t65706 = t15437 * t15506;
    let t65709 = t4965 * t5023;
    let t65716 = t65689 / F::cast_from(10368.0_f64) - F::cast_from(11.0_f64) / F::cast_from(486.0_f64) * t65691 + t15498 * t4974 / F::cast_from(108.0_f64) + t52575 / F::cast_from(162.0_f64) + F::cast_from(5.0_f64) / F::cast_from(10368.0_f64) * t52580 + F::cast_from(5.0_f64) / F::cast_from(1728.0_f64) * t52583 - t52586 / F::cast_from(1152.0_f64) + t44811 / F::cast_from(1296.0_f64) - t15594 * t4974 / F::cast_from(576.0_f64) - t52599 / F::cast_from(243.0_f64) - t65703 * t3511 / F::cast_from(144.0_f64) + t65706 * t3518 / F::cast_from(288.0_f64) + t65709 * t1232 / F::cast_from(216.0_f64) + t19083 * t3527 / F::cast_from(432.0_f64) + t19083 * t3531 / F::cast_from(216.0_f64);
    t65716
}
