//! MGGA_C_R2SCAN lxc pol — lxc_pol part 6 (v4rho4_1) CSE chunk 928/1462 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part6_v4rho4_1_chunk928<F: Float>(t106: F, t6595: F, t797: F, t97: F, t2262: F, t2266: F, t481: F, t1234: F, t288: F, t2858: F, t800: F, t292: F, t296: F, t297: F, t1249: F, t806: F, tau0: F) -> (F, F, F, F, F, F, F) {
    let t6598 = t97 * t106 * t6595 * t797;
    let t6599 = t2262 * t797;
    let t6601 = t2266 * t6599 * t481;
    let t6602 = 9.0 * t6601;
    let t6603 = t288 * t1234;
    let t6605 = t2858 * t6603 * t481;
    let t6606 = 18.0 * t6605;
    let t6608 = t800 * t800;
    let t6610 = 1.0 / t292 / t6608;
    let t6611 = tau0 * t6610;
    let t6621 = 1.0 / t297 / t296;
    let t6622 = t1249 * t806;
    (t6598, t6599, t6602, t6606, t6611, t6621, t6622)
}
