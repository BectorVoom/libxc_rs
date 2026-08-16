//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1374/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1374(t103063: f64, t103083: f64, t103224: f64, t103289: f64, t18431: f64, t27369: f64, t27459: f64, t28353: f64, t29358: f64, t29362: f64, t491: f64, t52852: f64, t5709: f64, t7908: f64, t7909: f64, t98623: f64, t98625: f64, t98627: f64, t98628: f64, t990: f64) -> f64 {
    let t103608 = -0.69505208333333333333e-3_f64 * t7908 * t103289 - 0.13901041666666666667e-2_f64 * t7908 * t103063 - 0.37134344353515625001e-4_f64 * t52852 * t491 * t990 * t28353 + 0.2782641015625e-3_f64 * t27369 * t103224 + 0.10203017057291666667e-2_f64 * t27369 * t103083 - t98623 + t98625 - t98627 - 0.92673611111111111112e-3_f64 * t98628 - 0.23168402777777777778e-3_f64 * t27459 * t29358 - 0.23168402777777777778e-3_f64 * t7908 * t5709 * t7909 * t18431 - 0.30891203703703703704e-3_f64 * t27459 * t29362;
    t103608
}
