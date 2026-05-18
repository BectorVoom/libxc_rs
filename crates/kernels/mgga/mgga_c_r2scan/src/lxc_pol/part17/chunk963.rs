//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 963/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk963<F: Float>(t3332: F, t7629: F, t7628: F, t8156: F, t6165: F, t8160: F, t7615: F, t7614: F, t3610: F, t6395: F, t8066: F, t2147: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t11640 = t3332 * t7629;
    let t11641 = t7628 * t11640;
    let t11643 = t3332 * t8156;
    let t11644 = t6165 * t11643;
    let t11646 = t3332 * t8160;
    let t11647 = t6165 * t11646;
    let t11649 = t3332 * t7615;
    let t11650 = t7614 * t11649;
    let t11652 = t6395 * t3610;
    let t11654 = t3332 * t8066;
    let t11655 = t2147 * t11654;
    (t11640, t11641, t11643, t11644, t11646, t11647, t11649, t11650, t11652, t11654, t11655)
}
