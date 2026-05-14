//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 1153/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk1153<F: Float>(t11363: F, t3974: F, t3990: F, t53592: F, t12204: F, t3989: F, t53283: F, t2409: F, t39460: F, t3965: F, t3972: F, t54499: F, t54590: F, t8884: F, t13984: F, t56104: F) -> (F, F, F, F, F) {
    let t56836 = t53592 * t3990 * t3974 * t11363;
    let t56840 = t3989 * t3990 * t53283 * t12204;
    let t56843 = t3965 * t2409 * t39460;
    let t56847 = t3972 * t54499 * t8884 * t54590;
    let t56849 = t56104 * t13984;
    (t56836, t56840, t56843, t56847, t56849)
}
