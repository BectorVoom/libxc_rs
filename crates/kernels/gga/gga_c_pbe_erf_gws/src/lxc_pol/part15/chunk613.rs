//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 613/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk613<F: Float>(t120: F, t2873: F, t102: F, t156: F, t974: F, t496: F, t481: F, t978: F, t128: F, t10: F, t501: F, t395: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2874 = t120 * t2873;
    let t2876 = F::new(0.2923025e1) * t102 * t2874;
    let t2878 = t156 * t974;
    let t2879 = t496 * t2878;
    let t2881 = t978 * t481;
    let t2885 = t128 * t2873;
    let t2886 = t10 * t2885;
    let t2890 = t501 * t978;
    let t2891 = t2890 * t395;
    (t2874, t2876, t2878, t2879, t2881, t2885, t2886, t2890, t2891)
}
