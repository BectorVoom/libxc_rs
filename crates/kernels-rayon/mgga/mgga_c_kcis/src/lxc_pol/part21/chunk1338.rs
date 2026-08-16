//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1338/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1338(t3668: f64, t8104: f64, t11230: f64, t1291: f64, t15101: f64, t2205: f64, t27136: f64, t28253: f64, t35615: f64, t3670: f64, t3699: f64, t47681: f64, t47700: f64, t8108: f64, t8117: f64, t92398: f64, t95483: f64, t95485: f64, t95487: f64, t95489: f64, t95491: f64, t95492: f64, t95495: f64, t95498: f64, t95500: f64, t95502: f64) -> f64 {
    let t96670 = t8104 * t3668;
    let t96689 = -12.0_f64 * t11230 * t1291 * t28253 - 6.0_f64 * t11230 * t3670 * t8117 - 6.0_f64 * t11230 * t3699 * t8108 + 24.0_f64 * t35615 * t3670 * t8108 - 6.0_f64 * t15101 * t92398 - t2205 * t47681 - 6.0_f64 * t27136 * t47700 + 2.0_f64 * t3670 * t96670 - t95483 - t95485 - t95487 - t95489 + t95491 + t95492 + t95495 - t95498 + t95500 + t95502;
    t96689
}
