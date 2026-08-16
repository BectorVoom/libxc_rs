//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 369/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk369<F: Float>(t3102: F, t3101: F, t3091: F, t464: F, t866: F, t869: F, t871: F, t1232: F) -> (F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t3103 = t3102 * pi;
    let t3104 = t3101 * t3103;
    let t3106 = t464 * t3091;
    let t3107 = t3106 * t866;
    let t3111 = t869 * t871;
    let t3113 = F::cast_from(1.0_f64) / t1232;
    (t3103, t3104, t3106, t3107, t3111, t3113)
}
