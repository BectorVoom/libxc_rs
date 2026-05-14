//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 750/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk750<F: Float>(t101: F, t5669: F, t159: F, t285: F, t4713: F, t1904: F, t477: F, t281: F, t1128: F, t780: F, t2700: F, t2703: F, t2709: F, t2712: F, t2739: F, t4385: F, t4386: F, t4388: F, t4390: F, t4392: F, t4395: F, t4396: F, t4399: F, t4400: F, t4402: F, t4404: F) -> (F, F, F, F, F, F, F) {
    let t5670 = t101 * t5669;
    let t5673 = t4713 * t159 * t285;
    let t5677 = t1904 * t477 * t285;
    let t5679 = 0.02394846802050922 * t281 * t5677;
    let t5681 = t780 * t1128 * t285;
    let t5682 = t281 * t5681;
    let t5684 = t4385 + t2700 + t2703 + t4386 - t2709 - t2712 + t4388 - t4390 - t4392 - t4395 - t2739 - t4396 + t4399 - t4400 + t4402 - t4404;
    (t5670, t5673, t5677, t5679, t5681, t5682, t5684)
}
