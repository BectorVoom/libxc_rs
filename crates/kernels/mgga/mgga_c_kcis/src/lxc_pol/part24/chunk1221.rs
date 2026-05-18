//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1221/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1221<F: Float>(t1021: F, t19862: F, t19879: F, t7754: F, t26891: F, t6693: F, t5026: F, t5068: F, t6717: F, t9532: F, t19870: F, t7748: F) -> (F, F, F, F, F, F) {
    let t99931 = t1021 * t19862;
    let t99933 = t7754 * t19879;
    let t99935 = t26891 * t6693;
    let t99937 = t5026 * t5068;
    let t99939 = t9532 * t6717;
    let t99941 = t7748 * t19870;
    (t99931, t99933, t99935, t99937, t99939, t99941)
}
