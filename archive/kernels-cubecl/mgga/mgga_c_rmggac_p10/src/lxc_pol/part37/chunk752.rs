//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 752/1128 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk752<F: Float>(t69701: F, t69819: F, t69860: F, t69865: F, t15015: F, t275: F, t15014: F, t290: F, t69995: F, t1347: F, t3286: F, t15017: F) -> (F, F, F, F, F, F, F, F, F) {
    let t73353 = F::cast_from(0.22800128353348965e-6_f64) * t69701;
    let t73375 = F::cast_from(0.19516036795685772889e-4_f64) * t69819;
    let t73382 = F::cast_from(0.69390353051327192495e-4_f64) * t69860;
    let t73383 = F::cast_from(0.13010691197123848593e-4_f64) * t69865;
    let t73395 = t275 * t15015;
    let t73397 = t290 * t15014;
    let t73411 = F::cast_from(0.17451485956252114153e-3_f64) * t69995;
    let t73420 = t1347 * t3286;
    let t73448 = t275 * t15017;
    (t73353, t73375, t73382, t73383, t73395, t73397, t73411, t73420, t73448)
}
