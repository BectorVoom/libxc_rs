//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1013/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1013<F: Float>(t35537: F, t681: F, t89: F, t140756: F, t140762: F, t27796: F, t33294: F, t141204: F, t141206: F, t141220: F, t141223: F, t141231: F, t141240: F, t141255: F, t141282: F, t141295: F, t141304: F, t150259: F, t150263: F, t150267: F, t150271: F) -> (F, F, F) {
    let t150277 = t89 * t681 * t35537;
    let t150283 = t140762 * t140756 * t33294 * t27796;
    let t150285 = -t141204 + t141206 / F::new(3.0) + F::new(2.0) * t141220 - F::new(4.0) / F::new(3.0) * t141223 + t150259 / F::new(9.0) + F::new(8.0) * t150263 - F::new(3.0) / F::new(8.0) * t150267 - t150271 / F::new(2.0) + F::new(4.0) / F::new(3.0) * t141231 - F::new(8.0) / F::new(3.0) * t141240 - t141255 / F::new(12.0) - F::new(2.0) / F::new(3.0) * t150277 - t141282 + t141295 / F::new(6.0) - t141304 / F::new(3.0) - F::new(6.0) * t150283;
    (t150277, t150283, t150285)
}
