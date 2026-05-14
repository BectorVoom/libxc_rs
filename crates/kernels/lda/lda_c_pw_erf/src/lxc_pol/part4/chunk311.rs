//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 311/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk311<F: Float>(t1039: F, t40: F, t344: F, t358: F, t391: F, t339: F, t1022: F, t379: F, t386: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1040 = t40 * t1039;
    let t1041 = 2.0 * t1040;
    let t1042 = t344 * t358;
    let t1043 = 8.0 * t1042;
    let t1044 = t344 * t391;
    let t1045 = 8.0 * t1044;
    let t1046 = t339 * t358;
    let t1047 = 8.0 * t1046;
    let t1048 = t339 * t391;
    let t1049 = 8.0 * t1048;
    let t1051 = t379 * t1022 * t386;
    (t1040, t1041, t1042, t1043, t1045, t1046, t1047, t1049, t1051)
}
