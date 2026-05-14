//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1073/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1073<F: Float>(t10760: F, t30637: F, t6085: F, t30643: F, t11678: F, t7601: F, t30140: F, t30856: F, t6093: F, t12495: F, t19872: F, t29775: F, t29951: F, t10872: F, t12498: F, t12492: F, t19883: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t43528 = t6085 * t10760 * t30637;
    let t43531 = t6085 * t10760 * t30643;
    let t43533 = t7601 * t11678;
    let t43536 = t6085 * t10760 * t30140;
    let t43539 = t6093 * t10760 * t30856;
    let t43541 = t19872 * t12495;
    let t43544 = t6093 * t10760 * t29775;
    let t43547 = t6093 * t10760 * t29951;
    let t43549 = t10872 * t12498;
    let t43551 = t19883 * t12492;
    (t43528, t43531, t43533, t43536, t43539, t43541, t43544, t43547, t43549, t43551)
}
