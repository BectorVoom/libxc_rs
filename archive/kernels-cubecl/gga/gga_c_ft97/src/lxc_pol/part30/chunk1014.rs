//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1014/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1014<F: Float>(t150228: F, t33282: F, t7512: F, t7515: F, t35541: F, t375: F, t89: F, t150233: F, t7511: F, t35525: F, t681: F, t150238: F, t33301: F) -> (F, F, F, F, F) {
    let t150288 = t33282 * t7512 * t7515 * t150228;
    let t150291 = t89 * t375 * t35541;
    let t150295 = t7511 * t7512 * t7515 * t150233;
    let t150298 = t89 * t681 * t35525;
    let t150302 = t7511 * t7512 * t33301 * t150238;
    (t150288, t150291, t150295, t150298, t150302)
}
