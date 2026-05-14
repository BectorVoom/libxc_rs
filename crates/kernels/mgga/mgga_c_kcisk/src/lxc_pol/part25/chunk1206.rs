//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1206/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1206<F: Float>(t11699: F, t724: F, t751: F, t5438: F, t11984: F, t772: F, t794: F, t11985: F, t1990: F, t5432: F, t5444: F, t12350: F, t798: F, t801: F, t12351: F, t2038: F) -> (F, F, F, F, F, F, F) {
    let t47033 = t724 / t11699 / t751;
    let t47648 = t5438 * t5438;
    let t47649 = 1.0 / t47648;
    let t48363 = t772 / t11984 / t794;
    let t48397 = t1990 * t11985;
    let t48448 = t5432 * t5444;
    let t48504 = t798 / t12350 / t801;
    let t48510 = t2038 * t12351;
    (t47033, t47649, t48363, t48397, t48448, t48504, t48510)
}
