//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 861/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk861<F: Float>(t3821: F, t668: F, t505: F, t2493: F, t2380: F, t2393: F, t200: F, t1609: F, t213: F, t1109: F, t2378: F, t2417: F, t679: F) -> (F, F, F, F, F, F) {
    let t13389 = t3821 * t668;
    let t13390 = t13389 * t505;
    let t13391 = t2493 * t13390;
    let t13394 = t2393 * t2380;
    let t13395 = t13394 * t200;
    let t13399 = t1609 * t213;
    let t13400 = t13399 * t1109;
    let t13401 = t2378 * t2380;
    let t13402 = t13401 * t200;
    let t13406 = t679 * t2417;
    (t13390, t13391, t13395, t13400, t13402, t13406)
}
