//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1083/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1083<F: Float>(t106053: F, t106061: F, t106063: F, t106065: F, t113214: F, t113217: F, t95673: F, t95674: F, t95675: F, t95678: F, t95680: F, t99035: F, t99044: F, t99050: F, t106080: F, t106082: F, t106090: F, t106102: F, t113222: F, t113226: F, t113228: F, t113230: F, t113232: F, t113235: F, t113237: F, t95684: F, t99091: F, t99113: F) -> (F, F) {
    let t115698 = -0.68598428988911579154e-3 * t106053 - 0.68026775414003982662e-1 * t99035 + 0.34299214494455789577e-3 * t106061 + 0.12004725073059526352e-1 * t106063 - 0.24009450146119052704e-1 * t106065 + 0.12196800674228478774e-3 * t99044 - t95673 + 3.0 / 8.0 * t113214 - 35.0 / 36.0 * t99050 - t95674 + t95675 + t95678 - t113217 / 24.0 - t95680;
    let t115712 = -t95684 - 0.15246000842785598468e-3 * t106080 - 7.0 / 8.0 * t106082 - t113222 / 2.0 + 7.0 / 24.0 * t106090 - 0.3658582879408617555e-2 * t99091 + 0.51448821741683684367e-1 * t113226 + 0.51448821741683684367e-2 * t113228 - 0.85748036236139473944e-3 * t113230 - 0.10289764348336736873e0 * t113232 - 0.54214778996945588151e-4 * t99113 - 0.85748036236139473944e-3 * t113235 - 0.51448821741683684367e-2 * t113237 - 0.17149607247227894789e-2 * t106102;
    (t115698, t115712)
}
