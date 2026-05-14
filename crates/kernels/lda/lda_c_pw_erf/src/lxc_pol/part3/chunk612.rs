//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 612/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk612<F: Float>(t1468: F, t3899: F, t1318: F, t1381: F, t581: F, t549: F, t1466: F, t1416: F, t656: F, t1419: F, t245: F, t646: F, t1426: F, t645: F, t1433: F, t1: F, t1578: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t3900 = t3899 * t1468;
    let t3901 = t1318 * t3900;
    let t3902 = 16.0 / 15.0 * t3901;
    let t3903 = t581 * t1381;
    let t3904 = t3903 * t549;
    let t3905 = t1466 * t3904;
    let t3907 = 4.0 / 5.0 * t1318 * t3905;
    let t3908 = t1416 * t656;
    let t3910 = t1419 * t656;
    let t3912 = t245 * t646;
    let t3915 = t245 * t1426;
    let t3917 = 2e-21 * t645 * t3915;
    let t3919 = 2.0 / 3.0 * t1433 * t656;
    let t3920 = t1578 * t1;
    (t3900, t3901, t3902, t3904, t3905, t3907, t3908, t3910, t3912, t3915, t3917, t3919, t3920)
}
