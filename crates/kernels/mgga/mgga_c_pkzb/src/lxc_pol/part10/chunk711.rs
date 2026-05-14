//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 711/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk711<F: Float>(t3160: F, t3161: F, t898: F, t1220: F, t904: F, t1167: F, t154: F, t2347: F, t385: F, t23: F, t2886: F) -> (F, F, F, F, F, F) {
    let t3162 = t3160 * t3161;
    let t3164 = 0.17315859105681463759e2 * t898 * t3162;
    let t3165 = t1220 * t904;
    let t3171 = t154 * t2347 * t1167;
    let t3172 = t385 * t3171;
    let t3174 = t23 * t2886;
    (t3162, t3164, t3165, t3171, t3172, t3174)
}
