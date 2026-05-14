//! GGA_C_GAPC lxc pol — lxc_pol part 38 (v4rho2sigma2_17) CSE chunk 391/1126 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part38_v4rho2sigma2_17_chunk391<F: Float>(t256: F, t62: F, t748: F, t1150: F, t19: F, t252: F, t1: F, t348: F, t745: F, t1165: F, t1167: F, t1169: F, t1154: F, t1161: F, t14: F, t351: F, t740: F) -> (F, F, F) {
    let t2056 = t256 * t256;
    let t2057 = 1.0 / t2056;
    let t2058 = t62 * t2057;
    let t2059 = t748 * t748;
    let t2063 = t1150 * t252 * t19;
    let t2067 = t348 * t745 * t1;
    let t2075 = -0.99474444444444444447e-4 * t1165 + 0.19894888888888888889e-3 * t1167 + 0.52442777777777777777e-2 * t1169;
    let t2078 = -t2063 * t1154 / 18.0 - t2067 * t351 / 6.0 + t740 * t1161 / 9.0 + t14 * t2075 / 2.0;
    (t2058, t2059, t2078)
}
