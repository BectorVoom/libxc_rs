//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 923/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk923<F: Float>(t1308: F, t4621: F, t28: F, t6455: F, t984: F, t1593: F, t4441: F, t35: F) -> (F, F, F, F, F, F) {
    let t29460 = t1308 * t4621;
    let t29461 = t28 * t29460;
    let t29464 = t6455 * t984;
    let t29465 = t28 * t29464;
    let t29468 = t1593 * t4441;
    let t29469 = t29468 * t35;
    (t29460, t29461, t29464, t29465, t29468, t29469)
}
