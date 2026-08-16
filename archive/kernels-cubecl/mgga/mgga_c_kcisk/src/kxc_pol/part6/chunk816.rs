//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 816/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk816<F: Float>(t14090: F, t7706: F, t1346: F, t8111: F, t1391: F, t8099: F, t443: F, t8105: F, t8108: F, t7737: F, t821: F) -> (F, F, F, F, F, F) {
    let t25544 = t14090 * t7706;
    let t25557 = t1346 * t8111;
    let t25559 = t1391 * t8099;
    let t25561 = t443 * t8105;
    let t25563 = t1346 * t8108;
    let t25590 = t821 * t7737;
    (t25544, t25557, t25559, t25561, t25563, t25590)
}
