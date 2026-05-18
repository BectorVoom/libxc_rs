//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 443/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk443<F: Float>(t1413: F, t1642: F, t1691: F, t11: F, t261: F, t50: F) -> (F, F, F, F) {
    let t1692 = t1642 * t1413;
    let t1693 = t1691 * t1692;
    let t1694 = t11 * t1693;
    let t1696 = t261 * t50;
    let t1697 = F::new(1.0) / t1696;
    (t1692, t1693, t1694, t1697)
}
