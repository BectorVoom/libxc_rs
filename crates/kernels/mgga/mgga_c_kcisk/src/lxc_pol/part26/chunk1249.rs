//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1249/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1249<F: Float>(t140: F, t31986: F, t975: F, t937: F, t15484: F, t2697: F, t9371: F, t3410: F, t397: F, t1128: F, t3376: F, t3411: F, t9379: F, t1123: F, t1129: F, t3417: F) -> (F, F, F, F, F, F) {
    let t111156 = t140 * t975 * t31986;
    let t111159 = t140 * t937 * t31986;
    let t111162 = t15484 * t9371 * t2697;
    let t111164 = t397 * t3410;
    let t111167 = t111164 * t3376 * t3411 * t1128;
    let t111168 = t9379 * t111167;
    let t111173 = t9379 * t111164 * t1129 * t1123 * t3417;
    (t111156, t111159, t111162, t111167, t111168, t111173)
}
