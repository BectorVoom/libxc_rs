//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 972/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk972<F: Float>(t19265: F, t19325: F, t19398: F, t19713: F, t1341: F, t1415: F, t1411: F, t3795: F, t3798: F, t5675: F, t3482: F, t5670: F, t5633: F, t3764: F, t5601: F, t1340: F) -> (F, F, F, F, F, F, F, F) {
    let t19715 = t19265 + t19325 + t19398 + t19713;
    let t19716 = t1341 * t19715;
    let t19717 = t1415 * t19716;
    let t19718 = t1411 * t19717;
    let t19720 = t3795 * t1341;
    let t19721 = t5675 * t3798;
    let t19722 = t19720 * t19721;
    let t19723 = t3482 * t19722;
    let t19725 = t5670 * t3798;
    let t19726 = t19720 * t19725;
    let t19727 = t5633 * t19726;
    let t19729 = t3764 * t5601;
    let t19730 = t1340 * t19729;
    (t19715, t19716, t19718, t19721, t19723, t19725, t19727, t19730)
}
