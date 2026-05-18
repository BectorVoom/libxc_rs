//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 520/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk520<F: Float>(t1780: F, t2: F, t2984: F, t1787: F, t2988: F, t463: F, t2993: F, t17: F, t3050: F, t9: F) -> (F, F, F, F, F, F) {
    let t3127 = t1780 * t2;
    let t3128 = t3127 * t2984;
    let t3131 = t1787 * t2988;
    let t3134 = t463 * t2;
    let t3135 = t3134 * t2993;
    let t3139 = t9 * t3050 * t17;
    (t3127, t3128, t3131, t3134, t3135, t3139)
}
