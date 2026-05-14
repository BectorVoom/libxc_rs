//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 930/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk930<F: Float>(t141206: F, t141220: F, t141223: F, t141231: F, t141240: F, t141255: F, t141282: F, t141295: F, t141304: F, t141607: F, t150259: F, t150263: F, t150267: F, t150271: F, t150277: F, t150283: F) -> (F,) {
    let t151296 = -t141607 + t141206 / 9.0 + 2.0 / 3.0 * t141220 - 4.0 / 9.0 * t141223 + t150259 / 27.0 + 8.0 / 3.0 * t150263 - t150267 / 8.0 - t150271 / 6.0 + 4.0 / 9.0 * t141231 - 8.0 / 9.0 * t141240 - t141255 / 36.0 - 2.0 / 9.0 * t150277 - t141282 / 3.0 + t141295 / 18.0 - t141304 / 9.0 - 2.0 * t150283;
    (t151296,)
}
