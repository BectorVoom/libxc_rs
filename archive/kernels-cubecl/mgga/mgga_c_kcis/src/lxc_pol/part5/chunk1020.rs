//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1020/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1020<F: Float>(t20: F, t284: F, t2194: F, t2909: F, t992: F, t1000: F, t1071: F, t2887: F, t2844: F, t110: F, t1705: F, t285: F) -> (F, F, F, F, F) {
    let t14393 = t284 * t20;
    let t14394 = t14393 * t2194;
    let t14395 = t992 * t2909;
    let t14400 = t992 * t1000;
    let t14401 = t14400 * t1071;
    let t14407 = t2887 * t1000;
    let t14408 = t14407 * t2844;
    let t14422 = t110 * t1705;
    let t14423 = t285 * t14422;
    (t14394, t14395, t14401, t14408, t14423)
}
