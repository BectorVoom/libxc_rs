//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 19 (v4rho3sigma_7) CSE chunk 417/1404 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part19_v4rho3sigma_7_chunk417<F: Float>(t505: F, t95: F, t510: F, t513: F, t137: F, t512: F, t131: F, t120: F, t133: F, t542: F, t1541: F, t242: F, t762: F) -> (F, F, F, F, F, F, F) {
    let t1563 = F::new(1.0) / t505 / t95;
    let t1572 = t510 * t513;
    let t1576 = F::new(1.0) / t512 / t137;
    let t1577 = t131 * t1576;
    let t1583 = F::new(0.38316777777777777777e0) * t133 * t542 * t120;
    let t1584 = t133 * t1541;
    let t1596 = F::new(0.16752564107100880375e0) * t762 * t242;
    (t1563, t1572, t1576, t1577, t1583, t1584, t1596)
}
