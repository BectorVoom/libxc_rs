//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1385/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1385<F: Float>(t1869: F, t2441: F, t33017: F, t62789: F, t112176: F, t35252: F, t1757: F, t8781: F, t9679: F, t1894: F, t9019: F, t35100: F, t5074: F, t1333: F, t35249: F, t112416: F, t116120: F, t116983: F, t121381: F, t32942: F, t33056: F, t34073: F, t34133: F, t34261: F, t35108: F, t35112: F, t9936: F, t9940: F) -> (F, F, F, F, F, F, F) {
    let t121882 = t1869 * t33017 * t62789 * t2441;
    let t121885 = t1869 * t112176 * t35252;
    let t121889 = t1869 * t9679 * t8781 * t1757;
    let t121893 = t1869 * t33017 * t9019 * t1894;
    let t121901 = t5074 * t35100;
    let t121903 = t1333 * t35249;
    let t121905 = 0.24125000000000000001e-1 * t33056 * t121381 - 0.69444444444444444446e-2 * t116120 * t9936 - 0.69444444444444444446e-2 * t116983 * t9936 + 0.13888888888888888889e-1 * t34073 * t34133 - 0.23280625000000000001e-2 * t112416 * t35108 - 0.33163888888888888888e-2 * t121882 - 0.33163888888888888888e-2 * t121885 + 0.73697530864197530862e-3 * t121889 + 0.55273148148148148147e-3 * t121893 + 0.20833333333333333334e-1 * t116983 * t9940 + 0.20833333333333333334e-1 * t34073 * t34261 - 0.20833333333333333334e-1 * t32942 * t35112 + 0.22109259259259259259e-2 * t121901 + 0.1621345679012345679e-1 * t121903;
    (t121882, t121885, t121889, t121893, t121901, t121903, t121905)
}
