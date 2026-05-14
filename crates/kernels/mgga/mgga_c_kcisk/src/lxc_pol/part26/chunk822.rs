//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 822/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk822<F: Float>(t1293: F, t3934: F, t394: F, t1319: F, t6174: F, t301: F, t342: F, t969: F, t119: F, t416: F, t1163: F, t1224: F) -> (F, F, F, F, F, F) {
    let t13493 = t1293 * t394 * t3934;
    let t13504 = t6174 * t1319;
    let t13522 = t342 * t969 * t301;
    let t13523 = 0.55403703703703703703e-1 * t13522;
    let t13524 = t119 * t416;
    let t13526 = t1224 * t13524 * t1163;
    (t13493, t13504, t13522, t13523, t13524, t13526)
}
