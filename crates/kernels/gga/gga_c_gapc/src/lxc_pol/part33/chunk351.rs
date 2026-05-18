//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 351/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk351<F: Float>(t1576: F, t1577: F, t1487: F, t465: F, t1423: F, t458: F, t1338: F, t437: F) -> (F, F, F, F) {
    let t1578 = t1576 * t1577;
    let t1581 = t1487 * t465;
    let t1584 = t1423 * t458;
    let t1587 = t437 * t1338;
    (t1578, t1581, t1584, t1587)
}
