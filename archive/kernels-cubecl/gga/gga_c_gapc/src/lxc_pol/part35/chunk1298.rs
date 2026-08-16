//! GGA_C_GAPC lxc pol — lxc_pol part 35 (v4rho2sigma2_14) CSE chunk 1298/1307 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part35_v4rho2sigma2_14_chunk1298<F: Float>(t11616: F, t3212: F, t10366: F, t11613: F, t3209: F, t11682: F, t23678: F, t2415: F, t2546: F, t11612: F, t2300: F, t3723: F) -> (F, F, F, F, F) {
    let t36009 = t3212 * t11616;
    let t36011 = t10366 * t11613;
    let t36013 = t3209 * t11616;
    let t36017 = t11682 * t2415 * t2546 * t23678;
    let t36020 = t11612 * t3723 * t2300;
    (t36009, t36011, t36013, t36017, t36020)
}
