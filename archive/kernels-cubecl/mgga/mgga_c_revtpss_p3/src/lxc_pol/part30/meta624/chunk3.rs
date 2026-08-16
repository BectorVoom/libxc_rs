//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2153/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2153<F: Float>(t14693: F, t25270: F, t14927: F, t27261: F, t93001: F, t92996: F, t92998: F, t93000: F, t99042: F, t99044: F, t99046: F, t99048: F, t99050: F, t99052: F) -> F {
    let t99054 = t25270 * t14693;
    let t99056 = t27261 * t14927;
    let t99058 = F::cast_from(0.1219527626469539185e-2_f64) * t93001;
    let t99059 = t99042 + F::cast_from(0.2032800112371413129e-4_f64) * t99044 + t99046 / F::cast_from(8.0_f64) + t99048 / F::cast_from(16.0_f64) - t92996 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t99050 + F::cast_from(0.17149607247227894789e-2_f64) * t99052 + F::cast_from(0.34299214494455789578e-2_f64) * t99054 + F::cast_from(0.25724410870841842183e-2_f64) * t99056 - t92998 + t93000 - t99058;
    t99059
}
