//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 852/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk852<F: Float>(t1380: F, t21484: F, t21489: F, t16969: F, t21485: F, t12135: F, t1368: F, t16830: F, t16925: F, t16935: F, t16940: F, t16944: F, t16954: F, t16981: F, t21470: F, t21474: F, t21478: F, t21480: F, t21486: F) -> (F,) {
    let t21491 = t21489 * t21484 * t1380;
    let t21494 = t16969 * t21485;
    let t21497 = -t1368 * t21470 / 16.0 + t1368 * t21474 / 24.0 - t21478 / 432.0 - t21480 / 162.0 - t12135 / 1296.0 - t16925 - t16935 - t16940 + t16944 + t16954 / 81.0 - t16981 - t16830 * t21486 / 108.0 + t16830 * t21491 / 72.0 + t16830 * t21494 / 72.0;
    (t21497,)
}
