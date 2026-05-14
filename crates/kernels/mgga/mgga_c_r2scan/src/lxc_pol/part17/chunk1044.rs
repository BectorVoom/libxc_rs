//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1044/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1044<F: Float>(t12495: F, t19872: F, t10760: F, t29775: F, t6093: F, t29951: F, t10872: F, t12498: F, t12492: F, t19883: F, t3179: F, t3344: F, t10879: F, t12503: F, t261: F, t3304: F, t9311: F) -> (F, F, F, F, F, F, F, F) {
    let t43541 = t19872 * t12495;
    let t43544 = t6093 * t10760 * t29775;
    let t43547 = t6093 * t10760 * t29951;
    let t43549 = t10872 * t12498;
    let t43551 = t19883 * t12492;
    let t43553 = t3179 * t3344;
    let t43555 = t10879 * t12503;
    let t43559 = t3304 * t261 * t9311;
    (t43541, t43544, t43547, t43549, t43551, t43553, t43555, t43559)
}
