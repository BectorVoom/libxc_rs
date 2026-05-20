//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1027/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1027<F: Float>(t1284: F, t1811: F, t1209: F, t1263: F, t3362: F, t12256: F, t13099: F, t1224: F, t140: F, t1789: F, t371: F, t676: F) -> (F, F, F, F, F, F) {
    let t17191 = t1284 * t1811;
    let t17192 = t1209 * t17191;
    let t17202 = t1263 * t3362;
    let t17235 = t13099 * t12256;
    let t17240 = t140 * t1224;
    let t17303 = t371 * t676 * t1789;
    (t17191, t17192, t17202, t17235, t17240, t17303)
}
