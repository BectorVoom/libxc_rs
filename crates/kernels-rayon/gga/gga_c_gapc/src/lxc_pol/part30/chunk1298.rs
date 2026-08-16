//! GGA_C_GAPC lxc pol — lxc_pol part 30 (v4rho2sigma2_9) CSE chunk 1298/1331 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part30_v4rho2sigma2_9_chunk1298(t3873: f64, t4905: f64, t12342: f64, t4908: f64, t33179: f64, t33182: f64, t33185: f64, t33187: f64, t33190: f64, t33193: f64, t33195: f64, t33197: f64, t33200: f64, t33203: f64, t33205: f64) -> (f64, f64, f64) {
    let t37644 = t4905 * t3873;
    let t37649 = 8.0_f64 * t4908 * t12342;
    let t37672 = 0.20596571349374880758e-4_f64 * t33179 + 0.2748593934505475288e-5_f64 * t33182 - 0.45018799441230669486e-7_f64 * t33185 - 0.66295654499063700024e-7_f64 * t33187 - 0.90037598882461338972e-7_f64 * t33190 + 0.67632724766374884054e-5_f64 * t33193 + 0.18550690221634253912e-3_f64 * t33195 - 0.18550690221634253912e-3_f64 * t33197 - 0.98326426188151041676e-8_f64 * t33200 + 0.12817159869818982005e-5_f64 * t33203 + 0.13505639832369200846e-5_f64 * t33205;
    (t37644, t37649, t37672)
}
