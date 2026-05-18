//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 638/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk638<F: Float>(t7367: F, t173: F, t322: F, t674: F, t797: F, t2252: F, t342: F, t344: F, t422: F) -> (F, F, F, F, F, F) {
    let t7368 = F::new(1.0) / t7367;
    let t7512 = t173 * t322;
    let t7513 = t674 * t674;
    let t7514 = F::new(1.0) / t7513;
    let t7639 = t797 * t797;
    let t7640 = F::new(1.0) / t7639;
    let t7704 = t342 * t2252 * t344 / F::new(18.0);
    let t7705 = t173 * t422;
    (t7368, t7512, t7514, t7640, t7704, t7705)
}
