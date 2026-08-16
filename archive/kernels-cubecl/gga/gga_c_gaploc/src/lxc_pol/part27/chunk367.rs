//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 367/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk367<F: Float>(t1603: F, t529: F, t1324: F, t531: F, t1265: F, t600: F, t568: F, t569: F, t417: F, t423: F, t566: F, t1305: F) -> (F, F, F, F, F, F, F) {
    let t1604 = t1603 * t529;
    let t1605 = t531 * t1324;
    let t1608 = t600 * t1265;
    let t1609 = t568 * t1608;
    let t1612 = t569 * t1265;
    let t1613 = t568 * t1612;
    let t1616 = t417 * t423;
    let t1617 = t1616 * t566;
    let t1620 = t569 * t1305;
    (t1604, t1605, t1609, t1613, t1616, t1617, t1620)
}
