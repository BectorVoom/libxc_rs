//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 674/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk674<F: Float>(t5607: F, t7195: F, t5587: F, t5608: F, t7178: F, t12: F, t397: F, t52: F, t428: F, t7318: F, t11: F, t1690: F, t53: F, t5555: F, t1693: F, t395: F) -> (F, F, F, F, F, F, F, F) {
    let t32179 = t7195 * t5607;
    let t32181 = 0.11352761063935582948e-3 * t5587 * t32179;
    let t32185 = 0.25537443351851851852e-1 * t7178 * t5608;
    let t32186 = t12 * t397;
    let t32187 = t52 * t32186;
    let t32190 = t7318 * t428;
    let t32208 = t1690 * t11 * t5555 * t53;
    let t32211 = t1693 * t395;
    (t32179, t32181, t32185, t32186, t32187, t32190, t32208, t32211)
}
