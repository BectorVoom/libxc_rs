//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 248/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk248<F: Float>(t286: F, t751: F, t159: F, t285: F, t535: F, t147: F, t545: F, t281: F, t532: F) -> (F, F, F, F, F) {
    let t753 = F::new(0.19957056683757681823e-1) * t751 * t286;
    let t755 = t535 * t159 * t285;
    let t759 = t147 * t545 * t285;
    let t761 = F::new(0.11974234010254609094e-1) * t281 * t759;
    let t762 = t532 * t147;
    (t753, t755, t759, t761, t762)
}
