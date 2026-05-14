//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 295/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk295<F: Float>(t279: F, t509: F, t516: F, t523: F, t527: F, t535: F, t549: F, t566: F, t574: F, t782: F, t791: F, t924: F, t928: F, t940: F, t944: F, t948: F, t980: F) -> (F,) {
    let t983 = -t509 + t516 - t523 - 0.54878743191129263322e-1 * t527 * t924 - 0.27439371595564631661e-1 * t535 * t928 - 0.43341108700271342816e-1 * t549 * t940 - 0.13002332610081402845e0 * t566 * t944 - 0.43341108700271342816e-1 * t574 * t948 + 0.43341108700271342816e-1 * t980 * t279 - t782 + t791;
    (t983,)
}
