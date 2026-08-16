//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1249/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1249<F: Float>(t1162: F, t13796: F, t2195: F, t3989: F, t3307: F, t875: F, t14127: F, t2503: F, t1118: F, t13859: F, t14682: F, t2158: F) -> (F, F, F, F) {
    let t53639 = t3989 * t13796 * t1162 * t2195;
    let t53643 = t3989 * t13796 * t3307 * t875;
    let t53645 = t14127 * t2503;
    let t53664 = t13859 * t14682 * t1118 * t2158;
    (t53639, t53643, t53645, t53664)
}
