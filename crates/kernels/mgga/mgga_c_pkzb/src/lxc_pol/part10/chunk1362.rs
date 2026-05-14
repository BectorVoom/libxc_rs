//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1362/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1362<F: Float>(t24: F, t1429: F, t1541: F, t1652: F, t1655: F, t18408: F, t2179: F, t22258: F, t23971: F, t3019: F, t3371: F, t3374: F, t4803: F, t507: F, t6097: F, t78: F, t7932: F, t8742: F, t9784: F, t9789: F, zeta_threshold: F) -> (F,) {
    let t90 = t24 <= zeta_threshold;
    let t27287 = piecewise3(t90, 0.0, 280.0 / 81.0 * t18408 * t3371 * t1652 + 224.0 / 27.0 * t7932 * t23971 - 28.0 / 27.0 * t9784 * t1655 + 32.0 / 9.0 * t2179 * t78 * t1541 - 16.0 / 9.0 * t3019 * t1429 + 16.0 / 3.0 * t3019 * t4803 - 28.0 / 27.0 * t6097 * t3374 * t1652 + 8.0 / 9.0 * t2179 * t8742 * t507 + 4.0 / 9.0 * t9789 * t1655 + t22258);
    (t27287,)
}
