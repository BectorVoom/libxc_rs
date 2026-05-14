//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 883/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk883<F: Float>(t285: F, t4422: F, t477: F, t1128: F, t1896: F, t405: F, t5669: F, t142: F, t5548: F, t455: F, t10833: F, t10843: F, t11482: F, t11486: F, t11495: F, t1733: F, t1735: F, t1881: F, t2208: F, t2211: F, t2806: F, t4283: F, t452: F, t456: F, t5490: F, t5783: F, t776: F, t8751: F, t9130: F) -> (F, F) {
    let t11498 = t4422 * t477 * t285;
    let t11499 = 0.0017434044910732151 * t11498;
    let t11501 = t1896 * t1128 * t285;
    let t11507 = t405 * t5669;
    let t11510 = t142 * t5548;
    let t11511 = t455 * t11510;
    let t11516 = 6.0 * t4283 * t776 * t456 - 0.16213771438917426 * t11482 + 3.0 * t10843 * t2208 + 9.0 * t1733 * t11486 - 6.0 * t1881 * t2806 + 9.0 * t2211 * t9130 - 0.0008717022455366076 * t11495 - t11499 - 0.0008717022455366076 * t11501 - 2.7743564462147594 * t8751 + 18.0 * t5490 * t452 * t2208 + 9.0 * t11507 * t1735 + 9.0 * t1733 * t11511 - 18.0 * t5783 * t10833;
    (t11510, t11516)
}
