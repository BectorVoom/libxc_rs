//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1374/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1374<F: Float>(t103063: F, t103083: F, t103224: F, t103289: F, t18431: F, t27369: F, t27459: F, t28353: F, t29358: F, t29362: F, t491: F, t52852: F, t5709: F, t7908: F, t7909: F, t98623: F, t98625: F, t98627: F, t98628: F, t990: F) -> F {
    let t103608 = -F::cast_from(0.69505208333333333333e-3_f64) * t7908 * t103289 - F::cast_from(0.13901041666666666667e-2_f64) * t7908 * t103063 - F::cast_from(0.37134344353515625001e-4_f64) * t52852 * t491 * t990 * t28353 + F::cast_from(0.2782641015625e-3_f64) * t27369 * t103224 + F::cast_from(0.10203017057291666667e-2_f64) * t27369 * t103083 - t98623 + t98625 - t98627 - F::cast_from(0.92673611111111111112e-3_f64) * t98628 - F::cast_from(0.23168402777777777778e-3_f64) * t27459 * t29358 - F::cast_from(0.23168402777777777778e-3_f64) * t7908 * t5709 * t7909 * t18431 - F::cast_from(0.30891203703703703704e-3_f64) * t27459 * t29362;
    t103608
}
