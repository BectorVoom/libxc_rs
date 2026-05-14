//! LDA_C_PW_ERF lxc pol — lxc_pol part 3 (v4rho4_1) CSE chunk 540/1138 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part3_v4rho4_1_chunk540<F: Float>(t1125: F, t31: F, t4: F, t1034: F, t357: F, t40: F, t379: F, t473: F, t1027: F, t155: F, t364: F, t988: F, t1010: F, t1953: F, t2061: F, t2717: F, t2720: F, t2723: F, t2728: F, t2730: F, t2732: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t3015 = t4 * t1125 * t31;
    let t3016 = 0.0034451131037037037 * t3015;
    let t3017 = t357 * t1034;
    let t3018 = t40 * t3017;
    let t3019 = 3.0 * t3018;
    let t3020 = t473 * t379;
    let t3027 = t155 * t1027;
    let t3031 = t473 * t364;
    let t3038 = t155 * t988;
    let t3046 = t155 * t1010;
    let t3058 = -4.7063 * t2717 + 3.1375333333333333 * t2720 - 3.6604555555555556 * t2723 - 1.6068111111111112 * t1953 + 0.2805166666666667 * t2728 - 0.5610333333333334 * t2730 - 0.6545388888888889 * t2732 - 0.4630888888888889 * t2061;
    (t3015, t3016, t3017, t3018, t3019, t3020, t3027, t3031, t3038, t3046, t3058)
}
