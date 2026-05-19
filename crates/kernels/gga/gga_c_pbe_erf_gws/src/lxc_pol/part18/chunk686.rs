//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 686/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk686<F: Float>(t343: F, t3854: F, t904: F, t916: F, t3187: F, t3780: F, param_a_c: F) -> (F, F, F, F) {
    let t3855 = t3854 * t343;
    let t3856 = t904 * t3855;
    let t3857 = t916 * t3856;
    let t3860 = F::new(7.0) / F::new(72.0) * t3187;
    let t3861 = param_a_c * t3780;
    (t3855, t3857, t3860, t3861)
}
