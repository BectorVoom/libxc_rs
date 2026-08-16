//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1959/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1959<F: Float>(t2018: F, t22125: F, t807: F, t6864: F, t94455: F, t26024: F, t6846: F, t22061: F, t25986: F, t2661: F, t22026: F, t94550: F) -> (F, F, F, F, F) {
    let t108587 = t807 * t2018 * t22125;
    let t108590 = t94455 * t6864;
    let t108592 = t26024 * t6846;
    let t108601 = t2661 * t25986 * t22061;
    let t108604 = t2661 * t94550 * t22026;
    (t108587, t108590, t108592, t108601, t108604)
}
