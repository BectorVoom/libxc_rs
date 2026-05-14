//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 838/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk838<F: Float>(t1354: F, t2822: F, t2841: F, t421: F, t4240: F, t4298: F, t10644: F, t118: F, t2778: F, t415: F, t2791: F, t399: F, t10524: F, t117: F, t84: F, t1338: F, t1347: F) -> (F, F, F, F, F, F, F, F) {
    let t10838 = 0.013871971944573394 * t2822 * t2841 * t1354;
    let t10840 = 0.12408369628826103 * t4240 * t421;
    let t10843 = 0.02267957317922317 * t4298 * t1354;
    let t10844 = t10644 * t118;
    let t10847 = 0.0004746123948660562 * t2778 * t415;
    let t10848 = t399 * t2791;
    let t10852 = 0.031505407223141116 * t84 * t10524 * t117;
    let t10853 = t1338 * t1347;
    (t10838, t10840, t10843, t10844, t10847, t10848, t10852, t10853)
}
