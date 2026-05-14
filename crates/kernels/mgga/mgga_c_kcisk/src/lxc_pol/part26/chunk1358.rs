//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1358/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1358<F: Float>(t119827: F, t9498: F, t27086: F, t33655: F, t119795: F, t119797: F, t119799: F, t119801: F, t119803: F, t119805: F, t119807: F, t119809: F, t119811: F, t119813: F, t119815: F, t119817: F, t119819: F, t119821: F, t119823: F, t119825: F) -> (F, F, F) {
    let t119828 = t119827 * t9498;
    let t119830 = t33655 * t27086;
    let t119832 = -t119795 / 48.0 + t119797 / 128.0 - 3.0 / 8.0 * t119799 + t119801 / 12.0 - t119803 / 9.0 - t119805 / 12.0 - t119807 / 24.0 + t119809 / 64.0 + t119811 / 36.0 - t119813 / 288.0 + t119815 / 18.0 + t119817 / 9.0 + t119819 / 16.0 + t119821 / 27.0 - t119823 / 72.0 + 2.0 / 9.0 * t119825 + t119828 / 24.0 + t119830 / 432.0;
    (t119828, t119830, t119832)
}
