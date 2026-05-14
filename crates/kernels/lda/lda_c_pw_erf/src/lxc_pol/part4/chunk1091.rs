//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1091/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1091<F: Float>(t15787: F, t15829: F, t15857: F, t15901: F, t173: F, t184: F, t199: F, t1496: F, t221: F, t2400: F, t11936: F, t11940: F, t15735: F, t15737: F, t15740: F, t15742: F, t15744: F, t15748: F, t15751: F, t15755: F, t15759: F, t15763: F, t15765: F, t15767: F, t9253: F) -> (F, F, F, F, F) {
    let t15907 = 2.0 / 15.0 * t173 * (t15787 + t15829 + t15857 + t15901) * t184 * t199;
    let t15911 = 4.0 / 15.0 * t2400 * t1496 * t184 * t221;
    let t15912 = 16.0 / 45.0 * t11936;
    let t15913 = 16.0 / 45.0 * t11940;
    let t15914 = -t15735 - t15737 + 4.0 / 3.0 * t9253 + t15740 - t15742 + t15744 + t15748 + t15751 + t15755 + t15759 + t15763 + t15765 - t15767 + t15907 + t15911 + t15912 - t15913;
    (t15907, t15911, t15912, t15913, t15914)
}
