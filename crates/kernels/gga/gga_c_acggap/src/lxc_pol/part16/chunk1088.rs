//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1088/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1088<F: Float>(t34159: F, t34162: F, t34171: F, t34173: F, t34176: F, t36970: F, t39049: F, t39052: F, t39054: F, t39057: F, t39060: F, t39062: F, t39064: F, t39069: F, t39071: F, t39073: F, t39075: F, t39077: F) -> F {
    let t39079 = F::cast_from(0.17149607247227894789e-2_f64) * t39049 - t34159 - F::cast_from(0.38586616306262763275e-1_f64) * t34162 - F::cast_from(0.17149607247227894789e-2_f64) * t39052 + F::cast_from(0.17149607247227894789e-2_f64) * t39054 + t39057 / F::cast_from(128.0_f64) + t39060 / F::cast_from(128.0_f64) - F::cast_from(0.5603125e-1_f64) * t39062 - F::cast_from(0.18868855373762491241e-2_f64) * t39064 - F::cast_from(0.12579236915841660827e-2_f64) * t39069 - F::cast_from(0.34299214494455789578e-2_f64) * t39071 - F::cast_from(0.25724410870841842183e-2_f64) * t39073 + F::cast_from(0.11321313224257494745e-1_f64) * t39075 - t34171 + t34173 + F::cast_from(0.17149607247227894789e-2_f64) * t39077 + t34176 - t36970;
    t39079
}
