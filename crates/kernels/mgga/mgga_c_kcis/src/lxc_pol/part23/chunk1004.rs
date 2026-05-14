//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1004/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1004<F: Float>(t28640: F, t7953: F, t28614: F, t28616: F, t28618: F, t28620: F, t28622: F, t28625: F, t28627: F, t28630: F, t28632: F, t28634: F, t28636: F, t28638: F, t28613: F, t1506: F) -> (F, F, F) {
    let t28641 = t28640 * t7953;
    let t28643 = t28614 / 16.0 + t28616 / 24.0 + t28618 / 128.0 + t28620 / 24.0 - t28622 / 72.0 - t28625 / 64.0 + t28627 / 96.0 - t28630 / 288.0 - t28632 / 6.0 - t28634 / 16.0 - t28636 / 24.0 + t28638 / 96.0 + t28641 / 24.0;
    let t28644 = t28613 + t28643;
    let t28645 = t1506 * t28644;
    (t28641, t28644, t28645)
}
