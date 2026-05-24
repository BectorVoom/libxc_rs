//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 635/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk635<F: Float>(t339: F, t63: F, t67: F, t378: F, t4143: F, t3951: F, t9: F, t403: F, t3936: F, t1310: F, t398: F, t1173: F, t476: F) -> (F, F, F, F, F, F, F) {
    let t6141 = t339 * t63 * t67;
    let t6142 = t378 * t4143;
    let t6174 = t9 * t3951;
    let t6175 = t6174 * t403;
    let t6183 = t3936 * t403;
    let t6204 = t1310 * t398;
    let t6256 = t476 * t1173;
    (t6141, t6142, t6174, t6175, t6183, t6204, t6256)
}
