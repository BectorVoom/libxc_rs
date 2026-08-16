//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1130/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1130<F: Float>(t1054: F, t5108: F, t7352: F, t10907: F, t2207: F, t3606: F, t10760: F, t24814: F, t6085: F, t25569: F, t6093: F, t22868: F, t24831: F) -> (F, F, F, F, F) {
    let t39677 = t5108 * t1054 * t7352;
    let t39686 = t2207 * t10907 * t3606;
    let t39689 = t6085 * t10760 * t24814;
    let t39692 = t6093 * t10760 * t25569;
    let t39695 = t22868 * t10760 * t24831;
    (t39677, t39686, t39689, t39692, t39695)
}
