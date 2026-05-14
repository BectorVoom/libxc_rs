//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 629/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk629<F: Float>(t3539: F, t604: F, t1882: F, t3324: F, t3327: F, t3320: F, t3339: F, t9065: F, t8796: F, t3343: F, t376: F, t89: F, t11402: F, t3330: F, t7773: F, t998: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12277 = t3539 * t604;
    let t12306 = t1882 * t3324;
    let t12307 = t12306 / 27.0;
    let t12308 = t1882 * t3327;
    let t12309 = 2.0 / 27.0 * t12308;
    let t12310 = t1882 * t3320;
    let t12311 = 2.0 / 81.0 * t12310;
    let t12327 = t1882 * t3339;
    let t12328 = t12327 / 27.0;
    let t12343 = 4.0 / 27.0 * t9065;
    let t12346 = 4.0 / 81.0 * t8796;
    let t12356 = t89 * t376 * t3343;
    let t12357 = 2.0 / 9.0 * t12356;
    let t12359 = t89 * t11402 * t3330;
    let t12362 = t89 * t7773 * t998;
    (t12277, t12306, t12307, t12308, t12309, t12310, t12311, t12327, t12328, t12343, t12346, t12356, t12357, t12359, t12362)
}
