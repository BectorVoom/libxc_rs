//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 816/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk816<F: Float>(t1128: F, t1155: F, t285: F, t343: F, t465: F, t159: F, t4137: F, t477: F, t169: F, t274: F, t301: F, t8363: F, t3309: F, t3310: F, t3318: F, t3319: F) -> (F, F, F, F, F, F, F) {
    let t8831 = 0.008135887625008338 * t1155 * t1128 * t285;
    let t8832 = t343 * t465;
    let t8834 = t8832 * t159 * t285;
    let t8838 = 0.026861343269868797 * t4137 * t477 * t285;
    let t8842 = 5.240451065072324 * t169 * t8363 * t274 * t301;
    let t8862 = 2.6116266666666665 * t3309 * t3310 * t343;
    let t8865 = 15.589466666666667 * t3318 * t3319 * t343;
    (t8831, t8832, t8834, t8838, t8842, t8862, t8865)
}
