//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 963/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk963<F: Float>(t193: F, t28954: F, t681: F, t7027: F, t1466: F, t6971: F, t317: F, t4129: F, t6222: F, t28835: F, t6223: F, t24964: F, t6970: F, t25410: F, t28931: F, t28935: F, t28941: F, t28947: F, t28951: F, t6210: F, t6216: F, t6263: F, t6267: F, t6963: F, t6972: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t28955 = t193 * t28954;
    let t28960 = t681 * t7027;
    let t28961 = t1466 * t28960;
    let t28963 = t681 * t6971;
    let t28964 = t1466 * t28963;
    let t28966 = t317 * t4129;
    let t28967 = t6222 * t28966;
    let t28968 = t193 * t28967;
    let t28971 = t28835 * t6223;
    let t28972 = t193 * t28971;
    let t28977 = t24964 * t6970;
    let t28978 = t193 * t28977;
    let t28981 = t6963 * t6267 / 6.0 + 4.0 * t28931 + t25410 / 9.0 + t6216 * t28935 / 9.0 + t6216 * t28941 / 9.0 - t6216 * t28947 / 27.0 + t6216 * t28951 / 9.0 + t1466 * t28955 / 6.0 + t6963 * t6263 / 6.0 - t28961 / 18.0 + t28964 / 9.0 - t1466 * t28968 / 3.0 - t1466 * t28972 / 3.0 - t6210 * t6972 / 3.0 - t1466 * t28978 / 3.0;
    (t28955, t28960, t28961, t28963, t28964, t28966, t28967, t28968, t28971, t28972, t28977, t28978, t28981)
}
