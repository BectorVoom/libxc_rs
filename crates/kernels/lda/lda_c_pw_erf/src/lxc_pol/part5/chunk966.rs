//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 966/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk966<F: Float>(t102: F, t411: F, t7914: F, t1697: F, t7913: F, t7919: F, t1832: F, t2615: F, t1844: F, t2610: F, t6121: F, t763: F, t127: F, t14797: F, t14803: F, t14814: F, t14817: F, t7918: F, t9037: F) -> (F, F, F, F, F, F) {
    let t20396 = 5.84605 * t102 * t7914 * t411;
    let t20397 = t1697 * t7913;
    let t20403 = 70.1526 * t102 * t7919 * t411;
    let t20406 = 52.61445 * t102 * t2615 * t1832;
    let t20409 = 17.53815 * t102 * t1844 * t2610;
    let t20412 = 17.53815 * t102 * t763 * t6121;
    let t20417 = 5.87616 * t14797 + t14803 + t14814 + t14817 + t20396 + 5.87616 * t127 * t20397 * t411 + t20403 - t20406 + t20409 + t20412 + 176.2848 * t127 * t9037 * t7918 * t411;
    (t20396, t20403, t20406, t20409, t20412, t20417)
}
