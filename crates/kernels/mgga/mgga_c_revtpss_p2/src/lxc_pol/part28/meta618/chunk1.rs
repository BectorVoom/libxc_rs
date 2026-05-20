//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2166/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2166<F: Float>(t99009: F, t99012: F, t99013: F, t99015: F, t99017: F, t99020: F, t99022: F, t99024: F, t99027: F, t99030: F, t99031: F, t99034: F, t99035: F) -> F {
    let t99037 = -F::cast_from(0.45351183609335988442e-1_f64) * t99009 + t99012 + F::cast_from(0.10841600599314203355e-2_f64) * t99013 + F::cast_from(0.17149607247227894789e-2_f64) * t99015 - F::cast_from(0.85748036236139473944e-3_f64) * t99017 + t99020 - t99022 - t99024 - t99027 + t99030 - F::cast_from(0.51448821741683684367e-1_f64) * t99031 + t99034 - F::cast_from(0.11337795902333997111e-1_f64) * t99035;
    t99037
}
