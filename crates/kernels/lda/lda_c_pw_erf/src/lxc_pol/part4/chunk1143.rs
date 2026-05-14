//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1143/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1143<F: Float>(t16822: F, t3965: F, t4501: F, t16785: F, t16786: F, t16787: F, t16788: F, t16789: F, t16790: F, t16792: F, t16794: F, t16798: F, t16801: F, t16805: F, t16808: F, t16811: F, t16814: F, t16818: F, t16820: F) -> (F, F) {
    let t16825 = 16.0 / 27.0 * t3965 * t4501 * t16822;
    let t16826 = t16785 + t16786 + t16787 + t16788 + t16789 - t16790 - t16792 + t16794 - t16798 + t16801 + t16805 - t16808 - t16811 - t16814 - t16818 + t16820 - t16825;
    (t16825, t16826)
}
