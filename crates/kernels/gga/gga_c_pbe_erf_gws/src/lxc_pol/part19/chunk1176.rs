//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1176/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1176<F: Float>(t14583: F, t50998: F, t53860: F, t1177: F, t1178: F, t12099: F, t371: F, t1167: F, t12275: F, t3928: F, t810: F, t14831: F, t30104: F, t14825: F, t3931: F, t3703: F, t944: F) -> (F, F, F, F, F, F, F, F) {
    let t57755 = t50998 * t53860 * t14583;
    let t57764 = t1177 * t371 * t1178 * t12099;
    let t57779 = t12275 * t1167;
    let t57785 = t3928 * t810;
    let t57803 = t30104 * t14831;
    let t57809 = t12275 * t14825;
    let t57820 = t3931 * t810;
    let t57830 = t3703 * t944;
    (t57755, t57764, t57779, t57785, t57803, t57809, t57820, t57830)
}
