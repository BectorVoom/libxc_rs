//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1189/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1189<F: Float>(t11002: F, t41189: F, t3269: F, t1044: F, t11323: F, t41153: F, t41156: F, t41158: F, t41160: F, t41162: F, t41165: F, t41168: F, t41170: F, t41173: F, t41176: F, t41179: F, t41182: F, t41185: F, t41188: F) -> (F, F) {
    let t41190 = t11002 * t41189;
    let t41192 = F::cast_from(5.0_f64) / F::cast_from(8.0_f64) * t3269 * t41190;
    let t41193 = t1044 * t11323 - t41153 - t41156 + t41158 + t41160 + t41162 + t41165 - t41168 - t41170 - t41173 - t41176 - t41179 - t41182 - t41185 + t41188 - t41192;
    (t41192, t41193)
}
