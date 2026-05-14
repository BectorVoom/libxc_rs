//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 953/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk953<F: Float>(t2206: F, t3799: F, t11583: F, t337: F, t6560: F, t2146: F, t1076: F, t810: F, t1123: F, t2255: F, t11464: F, t3235: F, t875: F, t11514: F, t2345: F, t6287: F) -> (F, F, F, F, F, F) {
    let t11922 = t2206 * t3799;
    let t11923 = 7.0 / 48.0 * t11922;
    let t11924 = t337 * t11583;
    let t11925 = t6560 * t11924;
    let t11927 = t2146 * t11925 / 16.0;
    let t11928 = t1076 * t810;
    let t11930 = t2255 * t1123 * t11928;
    let t11934 = t3235 * t11464 * t875;
    let t11938 = t2345 * t11514 * t6287;
    (t11923, t11924, t11927, t11930, t11934, t11938)
}
