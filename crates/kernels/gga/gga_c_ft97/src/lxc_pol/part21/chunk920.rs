//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 920/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk920<F: Float>(t1570: F, t165: F, t3188: F, t27420: F, t1360: F, t1642: F) -> (F, F, F, F) {
    let t27421 = t165 * t1570;
    let t27422 = t27421 * t3188;
    let t27423 = t27420 * t27422;
    let t27426 = t1642 * t1360;
    (t27421, t27422, t27423, t27426)
}
