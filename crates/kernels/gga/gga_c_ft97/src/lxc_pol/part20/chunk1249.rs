//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1249/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1249<F: Float>(t193: F, t2739: F, t28835: F, t89: F, t7068: F, t99314: F, t2680: F, t28719: F, t824: F, t1212: F, t98407: F, t2399: F, t7083: F, t28836: F, t681: F, t1477: F, t3704: F, t668: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t113577 = t89 * t193 * t28835 * t2739;
    let t113579 = t99314 * t7068;
    let t113580 = 2.0 / 27.0 * t113579;
    let t113581 = t2680 * t28719;
    let t113584 = t89 * t193 * t113581 * t824;
    let t113588 = t89 * t193 * t98407 * t1212;
    let t113591 = t89 * t2399 * t7083;
    let t113592 = 8.0 / 9.0 * t113591;
    let t113594 = t89 * t681 * t28836;
    let t113595 = 4.0 / 3.0 * t113594;
    let t113598 = t89 * t3704 * t1477 * t668;
    (t113577, t113579, t113580, t113581, t113584, t113588, t113591, t113592, t113594, t113595, t113598)
}
