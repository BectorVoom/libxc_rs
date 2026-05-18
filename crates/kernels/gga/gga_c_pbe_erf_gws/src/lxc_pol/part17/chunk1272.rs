//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1272/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1272<F: Float>(t1144: F, t4387: F, t859: F, t14136: F, t14420: F, t19906: F, t6683: F, t904: F, t14423: F, t3989: F, t8904: F, t4127: F, t4419: F) -> (F, F, F, F) {
    let t53699 = t859 * t1144 * t4387;
    let t53700 = t14136 * t53699;
    let t53704 = F::new(7.0) / F::new(72.0) * t19906 * t14420;
    let t53710 = t904 * t6683;
    let t53713 = t3989 * t53710 * t14423 * t8904;
    let t53715 = t4127 * t4419;
    (t53700, t53704, t53713, t53715)
}
