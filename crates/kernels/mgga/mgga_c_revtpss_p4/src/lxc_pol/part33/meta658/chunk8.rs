//! MGGA_C_REVTPSS lxc pol — lxc_pol part 33 (v4rho3sigma_8) CSE chunk 2124/2275 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk2124<F: Float>(t18622: F, t25245: F, t5989: F, t92978: F, t18634: F, t27261: F, t18334: F, t25270: F, t25277: F, t5985: F, t93021: F, t93035: F, t99066: F, t99070: F, t99074: F, t99078: F, t99086: F) -> F {
    let t106080 = t25245 * t18622;
    let t106082 = t92978 * t5989;
    let t106085 = t27261 * t18634;
    let t106088 = t25270 * t18334;
    let t106090 = t25277 * t5985;
    let t106092 = -t93021 - F::cast_from(0.25410001404642664113e-4_f64) * t106080 - F::cast_from(7.0_f64) / F::cast_from(48.0_f64) * t106082 - F::cast_from(0.80031500487063509015e-1_f64) * t99066 - t99070 + t99074 - t99078 + t99086 + F::cast_from(0.17149607247227894789e-2_f64) * t106085 + F::cast_from(0.27104001498285508387e-3_f64) * t93035 + F::cast_from(0.34299214494455789578e-2_f64) * t106088 + F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t106090;
    t106092
}
