//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 362/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk362<F: Float>(t137: F, t512: F, t131: F, t120: F, t133: F, t542: F, t242: F, t762: F, t528: F, t700: F, t1383: F, t148: F, t1472: F, t168: F, t270: F, t155: F, t266: F) -> (F, F, F, F, F, F, F, F) {
    let t1576 = 1.0 / t512 / t137;
    let t1577 = t131 * t1576;
    let t1583 = 0.38316777777777777777e0 * t133 * t542 * t120;
    let t1596 = 0.16752564107100880375e0 * t762 * t242;
    let t1601 = 0.16752564107100880375e0 * t528 * t700;
    let t1608 = 0.83762820535504401876e-1 * t148 * t1383;
    let t1611 = 0.53059442957798955448e-1 * t168 * t1472 * t270;
    let t1617 = t266 * t155;
    (t1576, t1577, t1583, t1596, t1601, t1608, t1611, t1617)
}
