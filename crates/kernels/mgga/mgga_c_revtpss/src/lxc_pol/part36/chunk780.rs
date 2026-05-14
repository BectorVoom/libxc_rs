//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 780/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk780<F: Float>(t555: F, t9990: F, t1432: F, t1433: F, t9288: F, t225: F, t9646: F, t1428: F, t22: F, t2452: F) -> (F, F, F, F, F) {
    let t10090 = t9990 * t555;
    let t10102 = 0.30356481678079769392e-1 * t1432 * t1433 * t9288;
    let t10111 = t9646 * t225;
    let t10114 = 0.19637199382202157274e-3 * t10111 * t1428 * t22;
    let t10115 = t22 * t2452;
    (t10090, t10102, t10111, t10114, t10115)
}
