//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 564/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk564<F: Float>(t1089: F, t175: F, t3355: F, t1036: F, t182: F, t315: F) -> (F, F, F) {
    let t3357 = t1089 * t175 * t3355;
    let t3358 = t1036 * t3357;
    let t3360 = t315 * t182;
    (t3357, t3358, t3360)
}
