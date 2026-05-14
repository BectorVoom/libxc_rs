//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1019/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1019<F: Float>(t28610: F, t7953: F, t28583: F, t28585: F, t28587: F, t28590: F, t28592: F, t28595: F, t28598: F, t28600: F, t28602: F, t28604: F, t28606: F, t28608: F, t573: F, t5998: F) -> (F, F, F) {
    let t28611 = t28610 * t7953;
    let t28613 = -t28583 / 24.0 + t28585 / 128.0 + t28587 / 18.0 - t28590 / 16.0 - t28592 / 128.0 + t28595 / 6.0 - t28598 / 16.0 + t28600 / 128.0 + t28602 / 8.0 - t28604 / 96.0 - t28606 / 24.0 - t28608 / 96.0 - t28611 / 9.0;
    let t28614 = t5998 * t573;
    (t28611, t28613, t28614)
}
