//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 727/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk727<F: Float>(t18396: F, t18429: F, t18454: F, t18511: F, t18599: F, t18639: F, t18706: F, t18754: F, t18587: F, t258: F, t113: F, t4375: F, t1274: F, t332: F, t992: F, t4380: F) -> (F, F, F, F) {
    let t18757 = t18396 + t18429 + t18454 + t18511 + t18599 + t18639 + t18706 + t18754;
    let t18760 = t18587 * t258;
    let t18794 = t113 * t4375;
    let t18795 = t1274 * t18794;
    let t18798 = t332 * t992;
    let t18799 = t4380 * t18798;
    (t18757, t18760, t18795, t18799)
}
