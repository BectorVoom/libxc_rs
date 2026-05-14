//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 1102/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk1102<F: Float>(t3873: F, t4905: F, t12342: F, t4908: F, t33179: F, t33182: F, t33185: F, t33187: F, t33190: F, t33193: F, t33195: F, t33197: F, t33200: F, t33203: F, t33205: F, t33209: F, t33212: F, t33214: F, t33217: F, t33221: F, t33226: F, t33228: F, t33230: F, t33232: F, t33240: F, t33242: F) -> (F, F, F, F) {
    let t37644 = t4905 * t3873;
    let t37649 = 8.0 * t4908 * t12342;
    let t37672 = 0.20596571349374880758e-4 * t33179 + 0.2748593934505475288e-5 * t33182 - 0.45018799441230669486e-7 * t33185 - 0.66295654499063700024e-7 * t33187 - 0.90037598882461338972e-7 * t33190 + 0.67632724766374884054e-5 * t33193 + 0.18550690221634253912e-3 * t33195 - 0.18550690221634253912e-3 * t33197 - 0.98326426188151041676e-8 * t33200 + 0.12817159869818982005e-5 * t33203 + 0.13505639832369200846e-5 * t33205;
    let t37685 = 0.27011279664738401692e-5 * t33209 + 0.7246363367825880434e-6 * t33212 - 0.40516919497107602538e-5 * t33214 + 0.3623181683912940217e-6 * t33217 + 0.33111854833537703651e-5 * t33221 + 0.23968194627773771045e-6 * t33226 - 0.5060221354166666667e-5 * t33228 + 0.37101380443268507824e-3 * t33230 + 0.30917817036057089854e-5 * t33232 - 0.48751922435761895589e-4 * t33240 - 0.13259130899812740005e-6 * t33242;
    (t37644, t37649, t37672, t37685)
}
