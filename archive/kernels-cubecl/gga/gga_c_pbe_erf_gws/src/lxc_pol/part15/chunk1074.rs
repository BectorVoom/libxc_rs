//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 1074/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk1074<F: Float>(t824: F, t938: F, t821: F, t13781: F, t3972: F, t2190: F, t3990: F, t3991: F, t3989: F, t332: F, t822: F) -> (F, F, F, F, F, F, F, F) {
    let t13782 = t824 * t938;
    let t13783 = t821 * t13782;
    let t13784 = t13781 * t13783;
    let t13785 = t3972 * t13784;
    let t13788 = t3990 * t3991 * t2190;
    let t13789 = t3989 * t13788;
    let t13791 = t824 * t332;
    let t13792 = t822 * t13791;
    (t13782, t13783, t13784, t13785, t13788, t13789, t13791, t13792)
}
