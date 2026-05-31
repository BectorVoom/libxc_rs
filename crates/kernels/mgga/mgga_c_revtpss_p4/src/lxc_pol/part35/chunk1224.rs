//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1224/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1224<F: Float>(t106080: F, t106082: F, t106090: F, t106102: F, t113222: F, t113226: F, t113228: F, t113230: F, t113232: F, t113235: F, t113237: F, t95684: F, t99091: F, t99113: F) -> F {
    let t115712 = -t95684 - F::cast_from(0.15246000842785598468e-3_f64) * t106080 - F::cast_from(7.0_f64) / F::cast_from(8.0_f64) * t106082 - t113222 / F::cast_from(2.0_f64) + F::cast_from(7.0_f64) / F::cast_from(24.0_f64) * t106090 - F::cast_from(0.3658582879408617555e-2_f64) * t99091 + F::cast_from(0.51448821741683684367e-1_f64) * t113226 + F::cast_from(0.51448821741683684367e-2_f64) * t113228 - F::cast_from(0.85748036236139473944e-3_f64) * t113230 - F::cast_from(0.10289764348336736873e0_f64) * t113232 - F::cast_from(0.54214778996945588151e-4_f64) * t99113 - F::cast_from(0.85748036236139473944e-3_f64) * t113235 - F::cast_from(0.51448821741683684367e-2_f64) * t113237 - F::cast_from(0.17149607247227894789e-2_f64) * t106102;
    t115712
}
