//! GGA_C_GAPC lxc pol — lxc_pol part 22 (v4rho2sigma2_1) CSE chunk 801/1209 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part22_v4rho2sigma2_1_chunk801<F: Float>(t10113: F, t787: F, t2238: F, t1055: F, t876: F, t3209: F, t10105: F, t1058: F, t2158: F, t798: F, t291: F, t653: F, t2418: F, t297: F, t2165: F, t3247: F) -> (F, F, F, F, F, F) {
    let t10114 = t10113 * t787;
    let t10115 = t2238 * t10114;
    let t10117 = t1055 * t876;
    let t10118 = t3209 * t10117;
    let t10120 = t10105 * t1058;
    let t10122 = t2158 * t798;
    let t10123 = t653 * t291;
    let t10125 = t10123 * t297 * t2418;
    let t10126 = t10122 * t10125;
    let t10128 = t2165 * t3247;
    (t10115, t10118, t10120, t10123, t10126, t10128)
}
