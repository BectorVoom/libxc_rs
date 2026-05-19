//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 936/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk936<F: Float>(t14210: F, t3293: F, t1035: F, t1670: F, t3074: F, t10314: F, t1662: F, t13495: F, t4579: F, t10324: F, t2944: F, t3255: F, t4572: F) -> (F, F, F, F, F, F, F) {
    let t14211 = t3293 * t14210;
    let t14215 = t1035 * t1670;
    let t14216 = t14215 * t3074;
    let t14217 = t3293 * t14216;
    let t14221 = t10314 * t1662 * t3074;
    let t14224 = t4579 * t13495;
    let t14228 = t10324 * t1662 * t2944;
    let t14232 = F::cast_from(0.13140859333333333334e-2_f64) * t3255 * t4572;
    (t14211, t14216, t14217, t14221, t14224, t14228, t14232)
}
