//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1164/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1164<F: Float>(t23281: F, t7045: F, t23257: F, t25262: F, t23285: F, t7038: F, t23342: F, t23289: F, t23253: F, t93062: F, t106080: F, t106082: F, t106090: F, t106102: F, t113222: F, t93021: F, t99091: F, t99113: F) -> (F,) {
    let t113226 = t7045 * t23281;
    let t113228 = t25262 * t23257;
    let t113230 = t7038 * t23285;
    let t113232 = t7045 * t23342;
    let t113235 = t7038 * t23289;
    let t113237 = t93062 * t23253;
    let t113240 = -t93021 - 0.76230004213927992339e-4 * t106080 - 7.0 / 16.0 * t106082 - t113222 / 4.0 + 7.0 / 48.0 * t106090 - 0.18292914397043087774e-2 * t99091 + 0.25724410870841842184e-1 * t113226 + 0.25724410870841842183e-2 * t113228 - 0.42874018118069736972e-3 * t113230 - 0.51448821741683684367e-1 * t113232 - 0.27107389498472794076e-4 * t99113 - 0.42874018118069736972e-3 * t113235 - 0.25724410870841842183e-2 * t113237 - 0.85748036236139473943e-3 * t106102;
    (t113240,)
}
