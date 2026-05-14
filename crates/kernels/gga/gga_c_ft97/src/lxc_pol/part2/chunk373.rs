//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 373/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk373<F: Float>(t2120: F, t579: F, t91: F, t1956: F, t1959: F, t1962: F, t1967: F, t1972: F, t1977: F, t1981: F, t1989: F, t2078: F, t2089: F, t143: F, t160: F, t376: F, t599: F, t89: F) -> (F, F, F, F, F) {
    let t2122 = t91 * t579 * t2120;
    let t2124 = 4.0 / 27.0 * t1956;
    let t2133 = -t2089 / 12.0 + t2122 / 6.0 + t2124 + 2.0 / 27.0 * t1959 + 2.0 / 9.0 * t1962 - 2.0 / 27.0 * t1967 + 2.0 / 9.0 * t1972 + 2.0 / 9.0 * t1977 - t1981 / 9.0 + 2.0 / 3.0 * t1989 - t2078 / 3.0;
    let t2135 = t143 * t2133 * t160;
    let t2140 = t89 * t376 * t599;
    (t2122, t2124, t2133, t2135, t2140)
}
