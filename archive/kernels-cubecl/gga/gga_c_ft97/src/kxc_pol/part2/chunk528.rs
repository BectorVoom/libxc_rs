//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 528/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk528<F: Float>(t432: F, t979: F, t452: F, t488: F, t492: F, t1852: F, t83: F, t1882: F, t981: F, t986: F, t110: F, t3103: F) -> (F, F, F, F, F, F, F, F) {
    let t3214 = t979 * t432;
    let t3216 = t452 * t488 * t3214;
    let t3219 = t979 * t492;
    let t3220 = t1852 * t3219;
    let t3221 = t83 * t3220;
    let t3224 = t1882 * t981;
    let t3227 = t452 * t986 * t432;
    let t3231 = t452 * t110 * t3103;
    (t3214, t3216, t3219, t3220, t3221, t3224, t3227, t3231)
}
