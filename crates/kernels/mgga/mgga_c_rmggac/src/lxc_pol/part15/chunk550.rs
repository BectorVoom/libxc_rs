//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 550/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk550<F: Float>(t1977: F, t1982: F, t7428: F, t1987: F, t2186: F, t1969: F, t7229: F) -> (F, F, F) {
    let t7430 = t1977 * t7428 * t1982;
    let t7431 = F::new(0.19863479950205658386e-4) * t7430;
    let t7438 = t2186 * t1987;
    let t7453 = t7229 * t1969;
    (t7431, t7438, t7453)
}
