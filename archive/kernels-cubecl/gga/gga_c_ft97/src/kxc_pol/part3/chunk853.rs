//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 853/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk853<F: Float>(t1775: F, t4765: F, t4768: F, t4759: F, t458: F, t4776: F, t4772: F, t16919: F, t24: F, t586: F, t16708: F, t2102: F) -> (F, F, F, F, F, F, F) {
    let t17272 = t1775 * t4765;
    let t17274 = t1775 * t4768;
    let t17276 = t1775 * t4759;
    let t17279 = t458 * t4776;
    let t17281 = t458 * t4772;
    let t17284 = t24 * t586 * t16919;
    let t17286 = t2102 * t16708;
    (t17272, t17274, t17276, t17279, t17281, t17284, t17286)
}
