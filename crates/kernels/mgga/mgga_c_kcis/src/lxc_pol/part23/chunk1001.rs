//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1001/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1001<F: Float>(t27520: F, t6029: F, t1552: F, t5752: F, t5932: F, t7948: F, t2066: F, t3738: F, t1928: F, t570: F, t7953: F, t28583: F, t28585: F, t28587: F, t28590: F, t28592: F, t28595: F, t28598: F, t28600: F) -> (F, F, F, F, F, F, F) {
    let t28602 = t27520 * t6029;
    let t28604 = t5752 * t1552;
    let t28606 = t7948 * t5932;
    let t28608 = t3738 * t2066;
    let t28610 = t570 * t1928;
    let t28611 = t28610 * t7953;
    let t28613 = -t28583 / 24.0 + t28585 / 128.0 + t28587 / 18.0 - t28590 / 16.0 - t28592 / 128.0 + t28595 / 6.0 - t28598 / 16.0 + t28600 / 128.0 + t28602 / 8.0 - t28604 / 96.0 - t28606 / 24.0 - t28608 / 96.0 - t28611 / 9.0;
    (t28602, t28604, t28606, t28608, t28610, t28611, t28613)
}
