//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1342/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1342<F: Float>(t118069: F, t2804: F, t34456: F, t9724: F, t118051: F, t33196: F, t34422: F, t5014: F, t116996: F, t33167: F, t34416: F, t34452: F, t9736: F, t118184: F, t7233: F, t112858: F, t34551: F, t9740: F) -> (F, F, F, F, F, F, F, F, F) {
    let t118324 = t2804 * t118069;
    let t118326 = t9724 * t34456;
    let t118330 = 0.13402777777777777778e-2 * t33196 * t118051;
    let t118334 = t5014 * t34422;
    let t118343 = 0.23214722222222222222e-2 * t116996;
    let t118348 = 0.11574074074074074074e-2 * t34416 * t33167;
    let t118355 = 0.34722222222222222222e-2 * t34452 * t9736;
    let t118360 = t7233 * t118184;
    let t118391 = 0.11574074074074074074e-2 * t9740 * t112858 * t34551;
    (t118324, t118326, t118330, t118334, t118343, t118348, t118355, t118360, t118391)
}
