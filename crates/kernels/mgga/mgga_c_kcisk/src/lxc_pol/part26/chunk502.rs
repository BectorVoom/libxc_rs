//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 502/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk502<F: Float>(t3571: F, t3657: F, t1202: F) -> (F, F, F, F) {
    let t3704 = 0.40256666666666666667e0 * t3571;
    let t3711 = 0.137975e0 * t3657;
    let t3721 = t1202 * t1202;
    let t3722 = 1.0 / t3721;
    (t3704, t3711, t3721, t3722)
}
