//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 1468/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk1468<F: Float>(t16869: F, t16910: F, t16979: F, t17020: F, t235: F, t5631: F, t814: F, t829: F, t252: F, t5611: F, t4182: F, t1499: F, t4280: F) -> (F, F, F, F, F, F) {
    let t17022 = t16869 + t16910 + t16979 + t17020;
    let t17023 = t235 * t17022;
    let t17027 = t814 * t5631;
    let t17028 = t17027 * t829;
    let t17030 = t252 * t5611;
    let t17031 = t17030 * t4182;
    let t17034 = t1499 * t4280;
    (t17022, t17023, t17028, t17030, t17031, t17034)
}
