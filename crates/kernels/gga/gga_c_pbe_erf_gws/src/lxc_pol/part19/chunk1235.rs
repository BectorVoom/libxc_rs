//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 1235/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk1235<F: Float>(t14425: F, t51563: F, t4138: F, t50948: F, t1114: F, t51922: F, t14001: F, t3214: F, t51819: F, t2370: F, t3958: F, t14784: F, t50994: F) -> (F, F, F, F, F, F, F) {
    let t53873 = t51563 * t14425;
    let t53886 = t50948 * t4138;
    let t53891 = t1114 * t51922;
    let t53896 = t14001 * t3214;
    let t53915 = F::new(119.0) / F::new(6912.0) * t51819;
    let t53923 = t3958 * t2370;
    let t53952 = t50994 * t14784;
    (t53873, t53886, t53891, t53896, t53915, t53923, t53952)
}
