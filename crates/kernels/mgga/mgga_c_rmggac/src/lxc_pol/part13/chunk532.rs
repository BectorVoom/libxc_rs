//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 532/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk532<F: Float>(t236: F, t7461: F, t7231: F, t1970: F, t209: F, t321: F, t476: F) -> (F, F, F) {
    let t7462 = t236 * t7461;
    let t7463 = t7231 * t7462;
    let t7464 = t1970 * t7463;
    let t7467 = t321 * t476 * t209;
    (t7463, t7464, t7467)
}
