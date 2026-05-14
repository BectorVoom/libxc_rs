//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 1147/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk1147<F: Float>(t1381: F, t4506: F, t6723: F, t6728: F, t954: F, t4515: F, t13812: F, t16843: F, t12338: F, t12356: F, t2480: F, t5041: F, t5045: F, t2104: F, t6867: F, t4073: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t16882 = 16.0 / 45.0 * t4506 * t6728 * t6723 * t1381;
    let t16883 = t6723 * t954;
    let t16886 = 16.0 / 45.0 * t4506 * t4515 * t16883;
    let t16889 = 16.0 / 9.0 * t4506 * t13812 * t16843;
    let t16890 = 64.0 / 135.0 * t12338;
    let t16891 = 16.0 / 135.0 * t12356;
    let t16893 = 4.0 / 15.0 * t5041 * t2480;
    let t16895 = 8.0 / 15.0 * t5045 * t2480;
    let t16897 = 8.0 / 15.0 * t2104 * t6867;
    let t16899 = 4.0 / 15.0 * t4073 * t2480;
    (t16882, t16883, t16886, t16889, t16890, t16891, t16893, t16895, t16897, t16899)
}
