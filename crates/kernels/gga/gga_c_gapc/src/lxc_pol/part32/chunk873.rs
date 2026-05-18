//! GGA_C_GAPC lxc pol — lxc_pol part 32 (v4rho2sigma2_11) CSE chunk 873/1311 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part32_v4rho2sigma2_11_chunk873<F: Float>(t2902: F, t6808: F, t3244: F, t291: F, t467: F, t787: F, t2238: F, t1055: F, t876: F, t3209: F, t10105: F, t1058: F) -> (F, F, F, F, F, F) {
    let t10110 = t2902 * t6808;
    let t10111 = t10110 * t3244;
    let t10113 = t467 * t291;
    let t10114 = t10113 * t787;
    let t10115 = t2238 * t10114;
    let t10117 = t1055 * t876;
    let t10118 = t3209 * t10117;
    let t10120 = t10105 * t1058;
    (t10110, t10111, t10113, t10115, t10118, t10120)
}
