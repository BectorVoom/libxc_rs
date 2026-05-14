//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 739/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk739<F: Float>(t6062: F, t771: F, t193: F, t1426: F, t2399: F, t1403: F, t1425: F, t2617: F, t6067: F, t681: F, t6063: F, t2354: F, t2409: F, t6003: F, t6008: F, t683: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t24207 = t6062 * t771;
    let t24208 = t193 * t24207;
    let t24211 = t2399 * t1426;
    let t24213 = 2.0 / 27.0 * t1403 * t24211;
    let t24216 = t1425 * t2617;
    let t24217 = t193 * t24216;
    let t24220 = t681 * t6067;
    let t24221 = t1403 * t24220;
    let t24223 = t681 * t6063;
    let t24224 = t1403 * t24223;
    let t24228 = t2354 * t6003 * t2409;
    let t24231 = t683 * t6008;
    (t24207, t24208, t24211, t24213, t24216, t24217, t24220, t24221, t24223, t24224, t24228, t24231)
}
