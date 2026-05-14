//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1213/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1213<F: Float>(t17391: F, t5916: F, t1534: F, t7385: F, t1533: F, t1529: F, t7389: F, t22212: F, t584: F, t583: F, t1546: F, t21971: F, t555: F, t578: F, t1543: F, t7275: F) -> (F, F, F, F, F, F) {
    let t22444 = t17391 * t5916;
    let t22446 = t7385 * t1534;
    let t22447 = t1533 * t22446;
    let t22449 = t1529 * t7389;
    let t22451 = t584 * t22212;
    let t22452 = t583 * t22451;
    let t22453 = t1546 * t22452;
    let t22455 = t555 * t21971;
    let t22456 = t583 * t22455;
    let t22457 = t578 * t22456;
    let t22459 = t1543 * t7275;
    (t22444, t22447, t22449, t22453, t22457, t22459)
}
