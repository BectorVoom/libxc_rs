//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2019/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2019<F: Float>(t106068: F, t106070: F, t106072: F, t106074: F, t95673: F, t95674: F, t95675: F, t95678: F, t95680: F, t99044: F, t99050: F, t99058: F, t99065: F) -> F {
    let t110421 = -F::cast_from(0.10289764348336736873e0_f64) * t106068 + F::cast_from(0.34299214494455789578e-1_f64) * t106070 - F::cast_from(0.85748036236139473944e-3_f64) * t106072 + F::cast_from(0.81312004494856525159e-4_f64) * t99044 - t95673 - F::new(35.0) / F::new(54.0) * t99050 - F::cast_from(0.85748036236139473944e-3_f64) * t106074 - t95674 + t95675 - t99058 + t95678 - t95680 - t99065;
    t110421
}
