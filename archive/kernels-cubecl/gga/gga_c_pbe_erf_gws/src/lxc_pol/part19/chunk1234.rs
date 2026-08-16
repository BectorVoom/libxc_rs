//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1234/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1234<F: Float>(t1112: F, t361: F, t51020: F, t3209: F, t51682: F, t3958: F, t6148: F, t352: F, t830: F, t1178: F, t8713: F, t2299: F, t371: F, t3970: F) -> (F, F, F, F, F, F) {
    let t53799 = t361 * t51020 * t1112;
    let t53806 = t51682 * t3209;
    let t53840 = t3958 * t6148;
    let t53841 = t830 * t352;
    let t53860 = t1178 * t8713;
    let t53865 = t3970 * t2299 * t371;
    (t53799, t53806, t53840, t53841, t53860, t53865)
}
