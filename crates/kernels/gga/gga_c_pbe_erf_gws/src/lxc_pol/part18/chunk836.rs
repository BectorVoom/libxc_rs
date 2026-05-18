//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 836/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk836<F: Float>(t7983: F, t1333: F, t960: F, t4753: F, t1326: F, t959: F, t40: F, t1444: F, t2506: F, t2513: F, t409: F, t2515: F, t414: F) -> (F, F, F, F, F, F, F) {
    let t7984 = F::new(8.0) * t7983;
    let t7986 = t1333 * t960;
    let t7994 = F::new(12.0) * t4753;
    let t7996 = t959 * t1326;
    let t7997 = t40 * t7996;
    let t8004 = t2506 * t1444;
    let t8010 = t409 * t2513;
    let t8011 = F::new(8.0) * t8010;
    let t8012 = t414 * t2515;
    (t7984, t7986, t7994, t7997, t8004, t8011, t8012)
}
