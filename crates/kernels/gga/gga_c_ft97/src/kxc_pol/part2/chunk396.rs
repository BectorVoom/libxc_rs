//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 396/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk396<F: Float>(t240: F, t668: F, t505: F, t231: F, t713: F, t1526: F, t2319: F, t2320: F, t342: F, t343: F, t719: F, t718: F, t10: F, t1542: F, t242: F, t375: F, t665: F) -> (F, F, F, F, F, F, F, F) {
    let t2321 = t240 * t668;
    let t2322 = t2321 * t505;
    let t2326 = t231 * t713;
    let t2330 = t719 - t2319 - t1526 * t2320 * t2322 / 12.0 - t342 * t343 * t2326 / 4.0;
    let t2331 = t2330 * t718;
    let t2334 = t10 * t1542 * t242;
    let t2335 = 2.0 / 27.0 * t2334;
    let t2336 = t375 * t665;
    (t2321, t2322, t2326, t2330, t2331, t2334, t2335, t2336)
}
