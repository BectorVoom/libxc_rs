//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1645/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1645<F: Float>(t14362: F, t2630: F, t1469: F, t749: F, t606: F, t4401: F, t4391: F, t705: F, t10446: F, t2375: F, t4186: F, t10457: F) -> (F, F, F, F, F, F, F, F) {
    let t14363 = t14362 * t2630;
    let t14369 = t749 * t1469;
    let t14370 = t14369 * t606;
    let t14372 = F::new(24.0) * t4401 * t14370;
    let t14386 = t705 * t4391;
    let t14401 = t10446 * t1469;
    let t14404 = t2375 * t4186;
    let t14413 = t10457 * t1469;
    (t14363, t14369, t14370, t14372, t14386, t14401, t14404, t14413)
}
