//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1023/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1023<F: Float>(t10907: F, t2207: F, t3606: F, t10760: F, t24814: F, t6085: F, t25569: F, t6093: F, t22868: F, t24831: F, t11717: F, t19883: F, t25997: F, t2147: F, t25573: F, t26997: F) -> (F, F, F, F, F, F, F, F) {
    let t39686 = t2207 * t10907 * t3606;
    let t39689 = t6085 * t10760 * t24814;
    let t39692 = t6093 * t10760 * t25569;
    let t39695 = t22868 * t10760 * t24831;
    let t39697 = t19883 * t11717;
    let t39700 = t6085 * t10760 * t25997;
    let t39703 = t2147 * t10760 * t25573;
    let t39706 = t6085 * t10760 * t26997;
    (t39686, t39689, t39692, t39695, t39697, t39700, t39703, t39706)
}
