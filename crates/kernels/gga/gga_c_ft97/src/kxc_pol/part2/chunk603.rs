//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 603/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk603<F: Float>(t2406: F, t458: F, t2410: F, t2414: F, t322: F, t668: F, t17: F, t2346: F, t667: F, t113: F, t170: F, t7512: F, t195: F, t25: F, t209: F, t2247: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t9560 = t458 * t2406;
    let t9562 = t458 * t2410;
    let t9564 = t458 * t2414;
    let t9567 = 1.0 / t322 / t668;
    let t9568 = t17 * t9567;
    let t9570 = 1.0 / t2346 / t667;
    let t9577 = 1.0 / t2346 / t113;
    let t9606 = 4.0 * t170 * t7512;
    let t9608 = 1.0 / t195 / t9606;
    let t9609 = t25 * t9608;
    let t9634 = t209 * t2247;
    (t9560, t9562, t9564, t9567, t9568, t9570, t9577, t9606, t9609, t9634)
}
