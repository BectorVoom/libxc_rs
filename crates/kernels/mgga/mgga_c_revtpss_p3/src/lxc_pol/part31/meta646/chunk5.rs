//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2117/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2117<F: Float>(t18394: F, t7025: F, t27221: F, t62403: F, t18352: F, t1945: F, t807: F, t61639: F, t99062: F, t61725: F, t103329: F, t103347: F, t93049: F, t93067: F, t93073: F, t93088: F, t99100: F, t99103: F) -> F {
    let t106093 = t7025 * t18394;
    let t106099 = t27221 * t62403;
    let t106102 = t807 * t1945 * t18352;
    let t106104 = t99062 * t61639;
    let t106106 = t27221 * t61725;
    let t106108 = -t106093 / F::cast_from(48.0_f64) - t103329 - F::cast_from(0.11337795902333997111e-1_f64) * t93049 + t99100 - t99103 - F::cast_from(0.45351183609335988444e-1_f64) * t93067 + F::cast_from(0.10841600599314203355e-2_f64) * t93073 - F::cast_from(0.15244095330869239812e-3_f64) * t93088 - t103347 + t106099 / F::cast_from(16.0_f64) - F::cast_from(0.28582678745379824648e-3_f64) * t106102 - t106104 / F::cast_from(4.0_f64) + t106106 / F::cast_from(8.0_f64);
    t106108
}
