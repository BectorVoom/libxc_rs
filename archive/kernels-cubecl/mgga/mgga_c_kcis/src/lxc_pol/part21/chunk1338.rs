//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1338/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1338<F: Float>(t3668: F, t8104: F, t11230: F, t1291: F, t15101: F, t2205: F, t27136: F, t28253: F, t35615: F, t3670: F, t3699: F, t47681: F, t47700: F, t8108: F, t8117: F, t92398: F, t95483: F, t95485: F, t95487: F, t95489: F, t95491: F, t95492: F, t95495: F, t95498: F, t95500: F, t95502: F) -> F {
    let t96670 = t8104 * t3668;
    let t96689 = -F::cast_from(12.0_f64) * t11230 * t1291 * t28253 - F::cast_from(6.0_f64) * t11230 * t3670 * t8117 - F::cast_from(6.0_f64) * t11230 * t3699 * t8108 + F::cast_from(24.0_f64) * t35615 * t3670 * t8108 - F::cast_from(6.0_f64) * t15101 * t92398 - t2205 * t47681 - F::cast_from(6.0_f64) * t27136 * t47700 + F::cast_from(2.0_f64) * t3670 * t96670 - t95483 - t95485 - t95487 - t95489 + t95491 + t95492 + t95495 - t95498 + t95500 + t95502;
    t96689
}
