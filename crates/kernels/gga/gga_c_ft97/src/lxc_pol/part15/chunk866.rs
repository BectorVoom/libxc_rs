//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 866/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk866<F: Float>(t1775: F, t22319: F, t22302: F, t22298: F, t22313: F, t22284: F, t21985: F, t2336: F, t89: F, t22330: F, t2755: F, t1882: F, t21946: F, t21982: F, t681: F, t21989: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t83463 = t1775 * t22319;
    let t83472 = t1775 * t22302;
    let t83474 = t1775 * t22298;
    let t83569 = t1775 * t22313;
    let t83587 = t1775 * t22284;
    let t83606 = t89 * t2336 * t21985;
    let t83615 = t2755 * t22330;
    let t83619 = t1882 * t21946;
    let t83652 = t89 * t681 * t21982;
    let t83655 = t89 * t2336 * t21989;
    (t83463, t83472, t83474, t83569, t83587, t83606, t83615, t83619, t83652, t83655)
}
