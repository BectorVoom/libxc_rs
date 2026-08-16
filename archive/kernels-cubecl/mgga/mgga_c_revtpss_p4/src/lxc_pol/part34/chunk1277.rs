//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1277/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1277<F: Float>(t27221: F, t76613: F, t23267: F, t7025: F, t106053: F, t106061: F, t106063: F, t106065: F, t92996: F, t92998: F, t93000: F, t93008: F, t93013: F, t99035: F, t99044: F, t99050: F) -> F {
    let t113214 = t27221 * t76613;
    let t113217 = t7025 * t23267;
    let t113219 = -F::cast_from(0.34299214494455789577e-3_f64) * t106053 - F::cast_from(0.34013387707001991332e-1_f64) * t99035 + F::cast_from(0.17149607247227894789e-3_f64) * t106061 + F::cast_from(0.60023625365297631762e-2_f64) * t106063 - F::cast_from(0.12004725073059526352e-1_f64) * t106065 + F::cast_from(0.60984003371142393869e-4_f64) * t99044 - t92996 + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t113214 - F::cast_from(35.0_f64) / F::cast_from(72.0_f64) * t99050 - t92998 + t93000 + t93008 - t113217 / F::cast_from(48.0_f64) - t93013;
    t113219
}
