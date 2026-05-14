//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 749/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk749<F: Float>(t1528: F, t8344: F, t4463: F, t8365: F, t3725: F, t7819: F, t1203: F, t7796: F, t1556: F, t8396: F, t8307: F, t1308: F, t3973: F, t8327: F, t1580: F, t8323: F, sigma0: F) -> (F, F, F, F, F, F, F, F) {
    let t27516 = t8344 * t1528;
    let t27584 = t8365 * t4463;
    let t27613 = t7819 * t3725;
    let t27627 = t7796 * t1203;
    let t27694 = t8396 * t1556;
    let t27705 = t8307 * sigma0;
    let t27706 = t27705 * t1308;
    let t27709 = t3973 * t8327;
    let t27710 = t1580 * t27709;
    let t27777 = t3973 * t8323;
    (t27516, t27584, t27613, t27627, t27694, t27706, t27710, t27777)
}
