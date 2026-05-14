//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 854/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk854<F: Float>(t164: F, t4130: F, t1155: F, t479: F, t1062: F, t8: F, t147: F, t483: F, t485: F, t1131: F, t4148: F, t1112: F, t717: F, t1138: F, t1597: F, t1586: F, t2910: F) -> (F, F, F, F, F, F, F, F) {
    let t10757 = t4130 * t164;
    let t10760 = 0.7561297733553868 * t1155 * t479;
    let t10762 = 1.0 / t8 / t1062;
    let t10764 = t10762 * t147 * t483;
    let t10766 = 7.439549289525431e-06 * t10764 * t485;
    let t10768 = t4148 * t1131 * t485;
    let t10770 = t717 * t1112;
    let t10772 = t10770 * t1138 * t1597;
    let t10775 = t1586 * t2910 * t485;
    (t10757, t10760, t10764, t10766, t10768, t10770, t10772, t10775)
}
