//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1146/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1146<F: Float>(t1328: F, t16863: F, t3965: F, t12121: F, t2471: F, t1314: F, t4488: F, t4490: F, t4749: F, t12064: F, t6734: F, t6737: F, t16830: F, t16833: F, t16836: F, t16839: F, t16842: F, t16846: F, t16851: F, t16854: F, t16856: F, t16860: F, t16862: F) -> (F, F, F, F, F, F) {
    let t16866 = 32.0 / 45.0 * t3965 * t16863 * t1328;
    let t16867 = t12121 * t2471;
    let t16870 = 16.0 / 15.0 * t4488 * t16867 * t1314;
    let t16873 = 32.0 / 45.0 * t4488 * t4490 * t4749;
    let t16874 = t12064 * t6734;
    let t16875 = 64.0 / 135.0 * t16874;
    let t16876 = t12064 * t6737;
    let t16877 = 32.0 / 81.0 * t16876;
    let t16878 = -t16830 + t16833 + t16836 + t16839 + t16842 - t16846 - t16851 - t16854 - t16856 + t16860 - t16862 + t16866 - t16870 + t16873 + t16875 - t16877;
    (t16866, t16870, t16873, t16875, t16877, t16878)
}
