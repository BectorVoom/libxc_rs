//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1163/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1163<F: Float>(t23267: F, t7025: F, t106053: F, t106061: F, t106063: F, t106065: F, t113214: F, t92996: F, t92998: F, t93000: F, t93008: F, t93013: F, t99035: F, t99044: F, t99050: F, t23263: F, t92981: F) -> (F, F) {
    let t113217 = t7025 * t23267;
    let t113219 = -0.34299214494455789577e-3 * t106053 - 0.34013387707001991332e-1 * t99035 + 0.17149607247227894789e-3 * t106061 + 0.60023625365297631762e-2 * t106063 - 0.12004725073059526352e-1 * t106065 + 0.60984003371142393869e-4 * t99044 - t92996 + 3.0 / 16.0 * t113214 - 35.0 / 72.0 * t99050 - t92998 + t93000 + t93008 - t113217 / 48.0 - t93013;
    let t113222 = t92981 * t23263;
    (t113219, t113222)
}
