//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 925/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk925<F: Float>(t1775: F, t5099: F, t5106: F, t16579: F, t738: F, t737: F, t18139: F, t192: F, t743: F, t458: F, t5118: F, t5114: F) -> (F, F, F, F, F, F, F) {
    let t18303 = t1775 * t5099;
    let t18305 = t1775 * t5106;
    let t18307 = t738 * t16579;
    let t18308 = t737 * t18307;
    let t18312 = t192 * t743 * t18139;
    let t18314 = t458 * t5118;
    let t18316 = t458 * t5114;
    (t18303, t18305, t18307, t18308, t18312, t18314, t18316)
}
