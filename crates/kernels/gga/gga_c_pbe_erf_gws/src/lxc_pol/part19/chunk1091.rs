//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1091/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1091<F: Float>(t3209: F, t51682: F, t3958: F, t6148: F, t352: F, t830: F, t1178: F, t8713: F, t2299: F, t371: F, t3970: F, t14425: F, t51563: F, t4138: F, t50948: F, t1114: F, t51922: F) -> (F, F, F, F, F, F, F, F) {
    let t53806 = t51682 * t3209;
    let t53840 = t3958 * t6148;
    let t53841 = t830 * t352;
    let t53860 = t1178 * t8713;
    let t53865 = t3970 * t2299 * t371;
    let t53873 = t51563 * t14425;
    let t53886 = t50948 * t4138;
    let t53891 = t1114 * t51922;
    (t53806, t53840, t53841, t53860, t53865, t53873, t53886, t53891)
}
