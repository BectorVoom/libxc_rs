//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 991/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk991<F: Float>(t3378: F, t6541: F, t6544: F, t1160: F, t1539: F, t19718: F, t180: F, t6068: F, t3077: F, t6489: F, t1410: F, t545: F, t3088: F, t4183: F, t6465: F, t6462: F) -> (F, F, F, F, F, F, F, F) {
    let t19741 = t3378 * t6541;
    let t19743 = t3378 * t6544;
    let t19746 = t1160 * t19718 * t1539;
    let t19748 = t180 * t6068;
    let t19752 = t3077 * t6489;
    let t19757 = t545 * t1410;
    let t19769 = t3088 * t6465 * t4183;
    let t19771 = t3077 * t6462;
    (t19741, t19743, t19746, t19748, t19752, t19757, t19769, t19771)
}
