//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 660/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk660<F: Float>(t1588: F, t942: F, t110: F, t8411: F, t979: F, t1871: F, t488: F, t10952: F, t83: F, t1882: F, t3268: F, t10992: F, t10976: F, t10981: F, t10985: F, t10990: F, t10996: F, t11000: F, t11005: F, t11010: F, t11015: F, t7822: F) -> (F, F, F, F, F) {
    let t11618 = t942 * t1588;
    let t11620 = t8411 * t110 * t11618;
    let t11623 = t979 * t1588;
    let t11625 = t1871 * t488 * t11623;
    let t11628 = t83 * t10952;
    let t11632 = 4.0 / 9.0 * t1882 * t3268;
    let t11638 = 2.0 / 27.0 * t10992;
    let t11644 = -2.0 / 27.0 * t7822 + 4.0 / 27.0 * t10976 + 2.0 / 9.0 * t10981 + t10985 / 9.0 + 2.0 / 27.0 * t10990 - t11638 + 4.0 / 9.0 * t10996 + 2.0 / 9.0 * t11000 + 8.0 / 9.0 * t11005 - 10.0 / 81.0 * t11010 - 8.0 / 27.0 * t11015;
    (t11620, t11625, t11628, t11632, t11644)
}
