//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 837/1097 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk837<F: Float>(t11914: F, t132: F, t1547: F, t1873: F, t5385: F, t588: F, t97: F, t4159: F, t871: F, t9424: F, t1992: F, t3457: F, t517: F, t5312: F, t1710: F, t830: F) -> (F, F, F, F, F, F, F, F) {
    let t11915 = t11914 / 45.0;
    let t11917 = t132 * t1547 * t1873;
    let t11918 = t11917 / 45.0;
    let t11920 = t5385 * t97 * t588;
    let t11921 = 0.36466666666666664 * t11920;
    let t11944 = t871 * t4159;
    let t11964 = 2e-21 * t9424;
    let t12006 = t1992 * t3457;
    let t12012 = t5312 * t517;
    let t12036 = t830 * t1710;
    (t11915, t11918, t11921, t11944, t11964, t12006, t12012, t12036)
}
