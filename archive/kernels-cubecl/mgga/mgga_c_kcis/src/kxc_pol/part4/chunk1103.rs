//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1103/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1103<F: Float>(t4708: F, t659: F, t13462: F, t2970: F, t26: F, t13467: F, t9714: F, t13516: F, t4714: F, t13475: F, t945: F, t13511: F) -> (F, F, F, F, F, F) {
    let t13912 = t659 * t4708;
    let t13914 = t2970 * t13462;
    let t13915 = t26 * t13914;
    let t13917 = t9714 * t13467;
    let t13918 = t26 * t13917;
    let t13920 = t2970 * t13516;
    let t13921 = t4714 * t13920;
    let t13923 = t945 * t13475;
    let t13924 = t26 * t13923;
    let t13926 = t945 * t13511;
    (t13912, t13915, t13918, t13921, t13924, t13926)
}
