//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1295/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1295<F: Float>(t3873: F, t4905: F, t12342: F, t4908: F, t33179: F, t33182: F, t33185: F, t33187: F, t33190: F, t33193: F, t33195: F, t33197: F, t33200: F, t33203: F, t33205: F) -> (F, F, F) {
    let t37644 = t4905 * t3873;
    let t37649 = F::cast_from(8.0_f64) * t4908 * t12342;
    let t37672 = F::cast_from(0.20596571349374880758e-4_f64) * t33179 + F::cast_from(0.2748593934505475288e-5_f64) * t33182 - F::cast_from(0.45018799441230669486e-7_f64) * t33185 - F::cast_from(0.66295654499063700024e-7_f64) * t33187 - F::cast_from(0.90037598882461338972e-7_f64) * t33190 + F::cast_from(0.67632724766374884054e-5_f64) * t33193 + F::cast_from(0.18550690221634253912e-3_f64) * t33195 - F::cast_from(0.18550690221634253912e-3_f64) * t33197 - F::cast_from(0.98326426188151041676e-8_f64) * t33200 + F::cast_from(0.12817159869818982005e-5_f64) * t33203 + F::cast_from(0.13505639832369200846e-5_f64) * t33205;
    (t37644, t37649, t37672)
}
