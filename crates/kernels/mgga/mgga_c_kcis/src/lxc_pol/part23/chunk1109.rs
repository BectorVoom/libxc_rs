//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1109/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1109<F: Float>(t28610: F, t7953: F, t28583: F, t28585: F, t28587: F, t28590: F, t28592: F, t28595: F, t28598: F, t28600: F, t28602: F, t28604: F, t28606: F, t28608: F) -> (F, F) {
    let t28611 = t28610 * t7953;
    let t28613 = -t28583 / F::cast_from(24.0_f64) + t28585 / F::cast_from(128.0_f64) + t28587 / F::cast_from(18.0_f64) - t28590 / F::cast_from(16.0_f64) - t28592 / F::cast_from(128.0_f64) + t28595 / F::cast_from(6.0_f64) - t28598 / F::cast_from(16.0_f64) + t28600 / F::cast_from(128.0_f64) + t28602 / F::cast_from(8.0_f64) - t28604 / F::cast_from(96.0_f64) - t28606 / F::cast_from(24.0_f64) - t28608 / F::cast_from(96.0_f64) - t28611 / F::cast_from(9.0_f64);
    (t28611, t28613)
}
