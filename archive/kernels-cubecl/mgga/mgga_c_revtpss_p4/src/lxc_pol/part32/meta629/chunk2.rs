//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 2021/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk2021<F: Float>(t103336: F, t103337: F, t106093: F, t106099: F, t106102: F, t106104: F, t106106: F, t93049: F, t93067: F, t93073: F, t93088: F, t99091: F, t99113: F) -> F {
    let t110441 = -t106093 / F::cast_from(24.0_f64) - F::cast_from(0.243905525293907837e-2_f64) * t99091 - F::cast_from(0.22675591804667994221e-1_f64) * t93049 + t103336 - t103337 - F::cast_from(0.90702367218671976884e-1_f64) * t93067 + F::cast_from(0.21683201198628406709e-2_f64) * t93073 - F::cast_from(0.30488190661738479625e-3_f64) * t93088 - F::cast_from(0.36143185997963725434e-4_f64) * t99113 + t106099 / F::cast_from(8.0_f64) - F::cast_from(0.57165357490759649296e-3_f64) * t106102 - t106104 / F::cast_from(2.0_f64) + t106106 / F::cast_from(4.0_f64);
    t110441
}
