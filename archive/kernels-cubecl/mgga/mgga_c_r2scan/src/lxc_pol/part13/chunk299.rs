//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 299/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk299<F: Float>(t279: F, t509: F, t516: F, t523: F, t527: F, t535: F, t549: F, t566: F, t574: F, t782: F, t791: F, t924: F, t928: F, t940: F, t944: F, t948: F, t980: F) -> F {
    let t983 = -t509 + t516 - t523 - F::cast_from(0.54878743191129263322e-1_f64) * t527 * t924 - F::cast_from(0.27439371595564631661e-1_f64) * t535 * t928 - F::cast_from(0.43341108700271342816e-1_f64) * t549 * t940 - F::cast_from(0.13002332610081402845e0_f64) * t566 * t944 - F::cast_from(0.43341108700271342816e-1_f64) * t574 * t948 + F::cast_from(0.43341108700271342816e-1_f64) * t980 * t279 - t782 + t791;
    t983
}
