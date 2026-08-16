//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 734/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk734<F: Float>(t117: F, t1540: F, t1614: F, t570: F, t1756: F, t333: F, t1587: F, t1652: F, t558: F, t551: F, t321: F, t1763: F) -> (F, F, F, F, F, F, F, F) {
    let t30221 = t1540 * t117;
    let t30283 = t1614 * t570;
    let t30311 = t1756 * t333;
    let t30344 = t1587 * t570;
    let t30360 = t558 * t1652;
    let t30400 = t551 * t1652;
    let t30453 = t1756 * t321;
    let t30490 = t1763 * t333;
    (t30221, t30283, t30311, t30344, t30360, t30400, t30453, t30490)
}
