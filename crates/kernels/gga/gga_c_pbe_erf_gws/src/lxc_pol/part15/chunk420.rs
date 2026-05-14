//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 420/1178 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk420<F: Float>(t138: F, t1570: F, t1572: F, t1577: F, t1578: F, t1590: F, t514: F, t520: F, t101: F, t242: F, t762: F, t145: F, t535: F) -> (F, F, F, F) {
    let t1592 = t138 * t1570 - 2.0 * t1572 * t520 + 2.0 * t1577 * t1578 - t1590 * t514;
    let t1593 = t101 * t1592;
    let t1596 = 0.16752564107100880375e0 * t762 * t242;
    let t1597 = t145 * t535;
    (t1592, t1593, t1596, t1597)
}
