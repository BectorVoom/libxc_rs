//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 1102/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk1102<F: Float>(t146: F, t6091: F, t774: F, t1541: F, t537: F, t252: F, t545: F, t6394: F, t6082: F, t776: F, t277: F, t6398: F) -> (F, F, F, F, F, F) {
    let t19872 = t146 * t6091 * t774;
    let t19875 = t1541 * t537;
    let t19877 = t146 * t19875 * t252;
    let t19883 = t545 * t6394;
    let t19886 = t776 * t6082;
    let t19890 = t6398 * t277;
    (t19872, t19875, t19877, t19883, t19886, t19890)
}
