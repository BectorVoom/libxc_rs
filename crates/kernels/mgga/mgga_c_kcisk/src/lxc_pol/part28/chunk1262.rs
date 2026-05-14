//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1262/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1262<F: Float>(t11985: F, t1990: F, t12350: F, t798: F, t801: F, t12351: F, t2038: F, t140: F, t4594: F, t5598: F, t5444: F, t7528: F, t164: F, t657: F, t2618: F, t642: F, t7069: F) -> (F, F, F, F, F, F, F, F) {
    let t48397 = t1990 * t11985;
    let t48504 = t798 / t12350 / t801;
    let t48510 = t2038 * t12351;
    let t60514 = t140 * t5598 * t4594;
    let t60823 = t7528 * t5444;
    let t62249 = t164 * t657;
    let t62760 = t2618 * t11985;
    let t62789 = t642 * t7069;
    (t48397, t48504, t48510, t60514, t60823, t62249, t62760, t62789)
}
