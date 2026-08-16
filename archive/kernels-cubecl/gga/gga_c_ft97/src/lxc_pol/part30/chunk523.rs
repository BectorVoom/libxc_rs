//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 523/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk523<F: Float>(t17694: F, t3700: F, t18: F, t2321: F, t342: F, t4910: F, t630: F, t231: F, t3821: F, t13616: F, t1526: F, t15567: F, t17685: F, t17688: F, t2320: F, t343: F, t3683: F, t3695: F, t3713: F, t3827: F, t9482: F, t9485: F, t9488: F) -> F {
    let t17695 = t17694 * t3700;
    let t17698 = t2321 * t18;
    let t17703 = t342 * t630 * t4910;
    let t17708 = t231 * t3821;
    let t17712 = t3683 + t3827 + t9482 - t9485 / F::cast_from(36.0_f64) - t9488 / F::cast_from(12.0_f64) - t17685 / F::cast_from(36.0_f64) - t15567 * t17688 / F::cast_from(9.0_f64) - t1526 * t2320 * t3695 / F::cast_from(12.0_f64) + t15567 * t17695 / F::cast_from(6.0_f64) + t1526 * t13616 * t17698 / F::cast_from(6.0_f64) - t17703 / F::cast_from(12.0_f64) - t1526 * t2320 * t3713 / F::cast_from(12.0_f64) - t342 * t343 * t17708 / F::cast_from(4.0_f64);
    t17712
}
