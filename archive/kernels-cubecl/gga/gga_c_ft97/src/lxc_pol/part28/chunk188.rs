//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 188/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk188<F: Float>(t108: F, t1308: F, t28: F, t110: F, t1307: F, t452: F, t457: F, t91: F, t26: F) -> (F, F, F, F, F) {
    let t1309 = t1308 * t108;
    let t1310 = t28 * t1309;
    let t1314 = t452 * t110 * t1307;
    let t1316 = t91 * t457;
    let t1317 = t1316 * t26;
    (t1309, t1310, t1314, t1316, t1317)
}
