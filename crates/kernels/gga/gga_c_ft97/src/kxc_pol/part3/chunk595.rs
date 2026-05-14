//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 595/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk595<F: Float>(t2281: F, t71: F, t118: F, t7911: F, t7944: F, t3626: F, t70: F, t170: F, t180: F, t645: F, t8640: F, t2252: F, t342: F, t511: F, t1526: F, t1944: F, t7705: F) -> (F, F, F, F, F, F, F, F) {
    let t8680 = t71 * t2281;
    let t8690 = 1.0 / t118 / t7911;
    let t8698 = 0.44934037037037037036e0 * t7944;
    let t8715 = t3626 * t70;
    let t8718 = 20.0 / 27.0 * t170 * t8715 * t180;
    let t8719 = t8640 * t645;
    let t8759 = t342 * t2252 * t511 / 18.0;
    let t8761 = t1526 * t7705 * t1944;
    (t8680, t8690, t8698, t8715, t8718, t8719, t8759, t8761)
}
