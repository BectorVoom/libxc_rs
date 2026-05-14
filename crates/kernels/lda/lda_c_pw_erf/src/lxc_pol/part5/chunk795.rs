//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 795/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk795<F: Float>(t8145: F, t918: F, t1184: F, t119: F, t321: F, t11: F, t174: F, t2: F, t2824: F, t2727: F, t343: F, t928: F, t328: F, t4606: F, t5021: F, t8141: F, t8143: F) -> (F, F, F, F, F, F, F) {
    let t8146 = t918 * t8145;
    let t8148 = t119 * t1184;
    let t8149 = t321 * t8148;
    let t8152 = f64::powf(t11, -2.5);
    let t8155 = t8152 * t2 * t2824 * t174;
    let t8157 = t2727 * t343;
    let t8159 = t928 * t8145;
    let t8161 = t328 * t8148;
    let t8164 = -2.8769444444444443 * t8141 + 27.618666666666666 * t8143 - 10.229135802469136 * t8146 + 8.950493827160495 * t8149 + 3.131074074074074 * t4606 + 0.0366775 * t8155 - 0.58684 * t8157 + 0.6520444444444444 * t8159 + 0.5705388888888889 * t8161 + 1.3490888888888888 * t5021;
    (t8146, t8149, t8155, t8157, t8159, t8161, t8164)
}
