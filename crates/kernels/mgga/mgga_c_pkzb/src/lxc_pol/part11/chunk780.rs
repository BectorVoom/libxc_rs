//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 780/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk780<F: Float>(t3340: F, t448: F, t459: F, t2528: F, t995: F, t3356: F, t2500: F, t2504: F, t3315: F, t3319: F, t3347: F, t34: F, t38: F, t445: F, t454: F, t6723: F, t6738: F, t8621: F, t8625: F, t8631: F, t8636: F, t8646: F, t8650: F, t8654: F, t8658: F, t991: F) -> (F, F, F, F, F, F) {
    let t8664 = t3340 * t448;
    let t8667 = t3340 * t459;
    let t8670 = t995 * t2528;
    let t8673 = t3356 * t448;
    let t8676 = t3356 * t459;
    let t8705 = -50.0 / 27.0 * t454 * t3315 - 10.0 / 27.0 * t34 * t8621 + 20.0 / 9.0 * t6723 * t8625 - 25.0 / 9.0 * t454 * t3319 + 10.0 / 9.0 * t34 * t8631 + 5.0 / 3.0 * t34 * t8636 + 200.0 / 27.0 * t3347 * t445 - 100.0 / 27.0 * t991 * t2500 + 50.0 / 9.0 * t991 * t2504 - 10.0 / 27.0 * t38 * t8646 - 20.0 / 9.0 * t6738 * t8650 + 10.0 / 9.0 * t38 * t8654 + 5.0 / 3.0 * t38 * t8658;
    (t8664, t8667, t8670, t8673, t8676, t8705)
}
