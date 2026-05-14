//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 768/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk768<F: Float>(t20: F, t2158: F, t394: F, t1220: F, t2075: F, t9447: F, t1312: F) -> (F, F, F, F) {
    let t9800 = t2158 * t394 * t20;
    let t9801 = t1220 * t9800;
    let t9804 = t9447 * t2075;
    let t9805 = t1312 * t9804;
    (t9800, t9801, t9804, t9805)
}
