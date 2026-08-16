//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1947/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1947<F: Float>(t29871: F, t7145: F, t1976: F, t6234: F, t6258: F, t1695: F, t7810: F) -> (F, F, F, F, F, F) {
    let t29872 = t7145 * t29871;
    let t29875 = t1976 * t6234;
    let t29876 = t7145 * t29875;
    let t29883 = t1976 * t6258;
    let t29884 = t7145 * t29883;
    let t29887 = t7810 * t1695;
    (t29872, t29875, t29876, t29883, t29884, t29887)
}
