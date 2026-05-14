//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 618/1120 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk618<F: Float>(t1060: F, t3613: F, t783: F, t1010: F, t3358: F, t1070: F, t2378: F, t1276: F, t1035: F, t352: F) -> (F, F, F, F, F, F) {
    let t3615 = t783 * t3613 * t1060;
    let t3625 = t3358 * t1010;
    let t3627 = t2378 * t1070;
    let t3629 = t1070 * t1010;
    let t3630 = t1276 * t3629;
    let t3675 = t352 * t1035;
    (t3615, t3625, t3627, t3629, t3630, t3675)
}
