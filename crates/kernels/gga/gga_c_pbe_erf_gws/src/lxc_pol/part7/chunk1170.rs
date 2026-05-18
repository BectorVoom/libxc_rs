//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1170/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk1170<F: Float>(t2133: F, t6106: F, t2138: F, t2263: F, t339: F, t824: F, t822: F, t20296: F, t6241: F, t2121: F, t337: F, t6180: F, t6217: F) -> (F, F, F) {
    let t20873 = t6106 * t2133;
    let t20875 = t20873 * t2138 / F::new(24.0);
    let t20876 = t339 * t2263;
    let t20877 = t824 * t20876;
    let t20878 = t822 * t20877;
    let t20879 = t20296 * t6241;
    let t20881 = t2121 * t337 * t20879;
    let t20883 = t20878 * t20881 / F::new(4.0);
    let t20885 = t6217 * t6180 / F::new(16.0);
    (t20875, t20883, t20885)
}
