//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1250/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1250<F: Float>(t24: F, t1429: F, t1541: F, t16250: F, t1651: F, t1652: F, t1655: F, t19680: F, t23971: F, t2548: F, t3371: F, t3374: F, t4803: F, t507: F, t5106: F, t6782: F, t78: F, t8734: F, t8739: F, t8742: F, zeta_threshold: F) -> (F,) {
    let t90 = t24 <= zeta_threshold;
    let t24594 = piecewise3(t90, 0.0, 40.0 / 81.0 * t16250 * t3371 * t1652 + 64.0 / 27.0 * t6782 * t23971 - 8.0 / 27.0 * t8734 * t1655 + 32.0 / 9.0 * t1651 * t78 * t1541 - 16.0 / 9.0 * t2548 * t1429 + 16.0 / 3.0 * t2548 * t4803 - 8.0 / 27.0 * t5106 * t3374 * t1652 + 8.0 / 9.0 * t1651 * t8742 * t507 + 4.0 / 9.0 * t8739 * t1655 - t19680);
    (t24594,)
}
