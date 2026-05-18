//! GGA_C_GAPC lxc pol — lxc_pol part 36 (v4rho2sigma2_15) CSE chunk 770/1328 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part36_v4rho2sigma2_15_chunk770<F: Float>(t173: F, t9184: F, t3170: F, t1027: F, t1996: F, t3100: F, t684: F, t1917: F, t1936: F, t628: F, t649: F, t3056: F) -> (F, F, F, F, F, F) {
    let t9185 = t9184 * t173;
    let t9186 = t3170 * t9185;
    let t9188 = t1027 * t1996;
    let t9190 = t3100 * t684;
    let t9192 = t1027 * t1917;
    let t9194 = t628 * t1936;
    let t9195 = t9194 * t649;
    let t9197 = t628 * t3056;
    (t9186, t9188, t9190, t9192, t9195, t9197)
}
