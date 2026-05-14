//! LDA_C_PMGB06 lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 902/1081 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part3_v4rho4_1_chunk902<F: Float>(t12092: F, t1440: F, t439: F, t2002: F, t3217: F, t3276: F, t3280: F, t1420: F, t4780: F, t12076: F, t12078: F, t12083: F, t12085: F, t12088: F, t12091: F, t1517: F, t1887: F) -> (F, F, F, F, F, F, F) {
    let t12095 = t439 * t12092 * t1440 / 9.0;
    let t12097 = t2002 * t3217 / 15.0;
    let t12099 = t2002 * t3276 / 15.0;
    let t12101 = t2002 * t3280 / 9.0;
    let t12103 = 2.0 / 15.0 * t1420 * t4780;
    let t12104 = -t12076 - t12078 - t12083 + t12085 - t12088 + t12091 + t12095 + t12097 + t12099 + t12101 + t12103;
    let t12105 = t1887 * t1517;
    (t12095, t12097, t12099, t12101, t12103, t12104, t12105)
}
