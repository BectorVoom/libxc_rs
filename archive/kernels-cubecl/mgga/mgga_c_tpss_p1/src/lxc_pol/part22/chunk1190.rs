//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1190/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1190<F: Float>(t17974: F, t803: F, t2391: F, t5559: F, t2395: F, t1705: F, t2398: F, t935: F, t5567: F, t5570: F) -> (F, F, F, F, F, F, F) {
    let t17975 = t17974 * t803;
    let t17976 = F::cast_from(7.0_f64) / F::cast_from(288.0_f64) * t17975;
    let t17977 = t5559 * t2391;
    let t17979 = t5559 * t2395;
    let t17990 = t1705 * t2398;
    let t17991 = t17990 * t935;
    let t17993 = t5567 * t5570;
    (t17975, t17976, t17977, t17979, t17990, t17991, t17993)
}
