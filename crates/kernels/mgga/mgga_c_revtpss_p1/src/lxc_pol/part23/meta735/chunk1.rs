//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2509/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2509<F: Float>(t14587: F, t2735: F, t40798: F, t826: F, t40593: F, t4452: F, t14933: F, t2482: F, t2668: F, t2719: F, t2710: F, t4371: F, t9732: F) -> (F, F, F, F) {
    let t50619 = t2735 * t40798 * t826 * t14587;
    let t50634 = t40593 * t4452;
    let t50681 = t2482 * t2719 * t2668 * t14933;
    let t50703 = t2710 * t9732 * t4371;
    (t50619, t50634, t50681, t50703)
}
