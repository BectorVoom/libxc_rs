//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1173/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1173<F: Float>(t113566: F, t25162: F, t28773: F, t7068: F, t99314: F, t2680: F, t28719: F, t2399: F, t7083: F, t89: F, t28836: F, t681: F, t28827: F, t28831: F, t1882: F, t28517: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t113567 = 2.0 / 9.0 * t113566;
    let t113568 = t25162 * t28773;
    let t113569 = 2.0 / 27.0 * t113568;
    let t113579 = t99314 * t7068;
    let t113581 = t2680 * t28719;
    let t113591 = t89 * t2399 * t7083;
    let t113594 = t89 * t681 * t28836;
    let t113595 = 4.0 / 3.0 * t113594;
    let t113601 = t89 * t681 * t28827;
    let t113602 = 4.0 / 3.0 * t113601;
    let t113609 = t89 * t681 * t28831;
    let t113610 = 4.0 / 3.0 * t113609;
    let t113631 = t1882 * t28517;
    (t113567, t113568, t113569, t113579, t113581, t113591, t113594, t113595, t113601, t113602, t113609, t113610, t113631)
}
