//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 839/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk839<F: Float>(t3205: F, t329: F, t838: F, t332: F, t6238: F, t863: F, t2079: F, t2112: F, t2153: F, t328: F, t6643: F, t824: F) -> (F, F, F, F, F, F) {
    let t8801 = t329 * t838 * t3205;
    let t8903 = t863 * t6238 * t332;
    let t8944 = t2079 * t2112;
    let t8967 = t863 * t2153 * t838;
    let t8986 = t6643 * t328;
    let t8987 = t824 * t8986;
    (t8801, t8903, t8944, t8967, t8986, t8987)
}
