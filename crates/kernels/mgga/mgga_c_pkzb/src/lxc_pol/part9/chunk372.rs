//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 372/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk372<F: Float>(t1410: F, t1413: F, t1414: F, t1444: F, t1449: F, t1450: F, t1466: F, t42: F, t430: F, t453: F, t55: F, t58: F, t63: F) -> (F, F) {
    let t1469 = F::new(0.165625e-1) * t1410 * t42 - F::new(0.6625e-1) * t1413 * t1414 + F::new(0.165625e-1) * t430 * t1444 + F::new(0.496875e-1) * t1449 * t1450 - F::new(0.165625e-1) * t453 * t1466;
    let t1475 = F::new(1.0) / t58 / t55 * t63;
    (t1469, t1475)
}
