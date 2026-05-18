//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 899/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk899<F: Float>(t1869: F, t28969: F, t22254: F, t2537: F, t10812: F, t28368: F, t5006: F, t2364: F, t8814: F, t11179: F, t2464: F, t8514: F) -> (F, F, F, F, F) {
    let t28970 = t1869 * t28969;
    let t28972 = t22254 * t2537;
    let t28973 = t1869 * t28972;
    let t28977 = t10812 * t28368;
    let t28978 = t5006 * t28977;
    let t28991 = t2364 * t8814;
    let t28992 = t11179 * t28991;
    let t28995 = t8514 * t2464;
    (t28970, t28973, t28978, t28992, t28995)
}
