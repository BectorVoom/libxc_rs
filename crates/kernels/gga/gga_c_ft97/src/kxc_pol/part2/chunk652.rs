//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 652/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk652<F: Float>(t110: F, t11392: F, t452: F, t1882: F, t3257: F, t1786: F, t11397: F, t463: F, t488: F, t1911: F, t2992: F, t10965: F, t83: F, t1825: F, t3214: F, t11430: F, t11432: F, t11436: F, t11439: F, t11444: F, t11448: F, t11451: F, t11455: F, t11459: F, t1901: F, t446: F) -> (F,) {
    let t11463 = t452 * t110 * t11392;
    let t11467 = 2.0 / 9.0 * t1882 * t3257;
    let t11468 = t1786 * t110;
    let t11469 = t11468 * t11397;
    let t11472 = t463 * t488;
    let t11473 = t2992 * t1911;
    let t11474 = t11472 * t11473;
    let t11477 = t83 * t10965;
    let t11481 = t452 * t1825 * t3214;
    let t11484 = t11430 - 2.0 / 9.0 * t1901 * t11432 - t11436 - 2.0 / 3.0 * t1901 * t11439 + t1901 * t11444 / 9.0 - t11448 + t1901 * t11451 / 9.0 - t446 * t11455 / 3.0 - 2.0 / 3.0 * t446 * t11459 - t446 * t11463 / 3.0 + t11467 - 4.0 / 9.0 * t1901 * t11469 - 4.0 / 9.0 * t1901 * t11474 + 2.0 / 3.0 * t446 * t11477 + 2.0 / 3.0 * t446 * t11481;
    (t11484,)
}
