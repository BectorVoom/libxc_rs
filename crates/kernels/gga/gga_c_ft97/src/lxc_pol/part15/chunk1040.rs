//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1040/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1040<F: Float>(t845: F, t90359: F, t90379: F, t90421: F, t90464: F, t91: F, t44121: F, t71276: F, t71298: F, t71305: F, t71319: F, t83728: F, t83770: F, t83772: F, t83781: F, t83789: F, t83792: F, t90326: F, t90330: F, t90335: F) -> (F, F) {
    let t90468 = t91 * t845 * (t90359 + t90379 + t90421 + t90464);
    let t90478 = 40.0 / 81.0 * t83728 - 15.0 / 16.0 * t90326 - 3.0 / 4.0 * t90330 + 16.0 / 9.0 * t71276 + t44121 + 8.0 * t90335 + t90468 / 2.0 + 4.0 / 3.0 * t83770 - 8.0 / 9.0 * t83772 + 8.0 / 3.0 * t83781 - 8.0 / 3.0 * t83789 + 8.0 * t83792 - 16.0 / 27.0 * t71298 + 16.0 / 9.0 * t71305 - 8.0 / 9.0 * t71319;
    (t90468, t90478)
}
