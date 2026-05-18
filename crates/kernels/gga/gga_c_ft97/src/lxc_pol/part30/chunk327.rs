//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 327/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk327<F: Float>(t332: F, t4375: F, t113: F, t1273: F, t909: F, t1274: F, t505: F, t910: F, t992: F, t18: F, t1577: F, t2321: F) -> (F, F, F, F, F, F) {
    let t4376 = t4375 * t332;
    let t4377 = t4376 * t113;
    let t4380 = t1273 * t909;
    let t4381 = t332 * t113;
    let t4382 = t4380 * t4381;
    let t4385 = t1274 * t505;
    let t4391 = t910 * t992;
    let t4394 = t332 * t18;
    let t4395 = t4394 * t1577;
    let t4906 = t2321 * t992;
    (t4377, t4382, t4385, t4391, t4395, t4906)
}
