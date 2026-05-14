//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 964/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk964<F: Float>(t2001: F, t5821: F, t1998: F, t5569: F, t1967: F, t9554: F, t6161: F, t7561: F, t34159: F, t34162: F, t34171: F, t34173: F, t34176: F, t36970: F, t39049: F, t39052: F, t39054: F, t39057: F, t39060: F, t39062: F, t39064: F, t39069: F) -> (F,) {
    let t39071 = t2001 * t5821;
    let t39073 = t1998 * t5569;
    let t39075 = t1967 * t9554;
    let t39077 = t7561 * t6161;
    let t39079 = 0.17149607247227894789e-2 * t39049 - t34159 - 0.38586616306262763275e-1 * t34162 - 0.17149607247227894789e-2 * t39052 + 0.17149607247227894789e-2 * t39054 + t39057 / 128.0 + t39060 / 128.0 - 0.5603125e-1 * t39062 - 0.18868855373762491241e-2 * t39064 - 0.12579236915841660827e-2 * t39069 - 0.34299214494455789578e-2 * t39071 - 0.25724410870841842183e-2 * t39073 + 0.11321313224257494745e-1 * t39075 - t34171 + t34173 + 0.17149607247227894789e-2 * t39077 + t34176 - t36970;
    (t39079,)
}
