//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 611/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk611<F: Float>(t3358: F, t826: F, t1070: F, t1271: F, t1276: F, t502: F) -> (F, F, F, F, F) {
    let t3359 = t3358 * t826;
    let t3361 = t1271 * t1070;
    let t3363 = t1070 * t826;
    let t3364 = t1276 * t3363;
    let t3366 = param_eta * t502;
    (t3359, t3361, t3363, t3364, t3366)
}
