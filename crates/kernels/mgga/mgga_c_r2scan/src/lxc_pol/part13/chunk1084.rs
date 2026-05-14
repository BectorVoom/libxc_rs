//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 1084/1115 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk1084<F: Float>(t10615: F, t11486: F, t3262: F, t1563: F, t3574: F, t10997: F, t10610: F, t10918: F, t11509: F, t3579: F, t36969: F, t3261: F, t498: F, t97: F, t32212: F, t481: F) -> (F, F, F, F, F, F) {
    let t40619 = 15.0 / 8.0 * t3262 * t10615 * t11486;
    let t40620 = t3574 * t1563;
    let t40623 = 135.0 / 64.0 * t3262 * t10997 * t40620;
    let t40626 = 3.0 * t10610 * t10918 * t11509;
    let t40628 = 45.0 / 64.0 * t3579 * t36969;
    let t40630 = t97 * t3261 * t498;
    let t40631 = t32212 * t481;
    (t40619, t40623, t40626, t40628, t40630, t40631)
}
