//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 651/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk651<F: Float>(t10088: F, t511: F, t7231: F, t3351: F, t570: F, t618: F) -> (F, F, F) {
    let t10089 = t511 * t10088;
    let t10090 = t7231 * t10089;
    let t10091 = t3351 * t10090;
    let t10092 = 0.25538759935978703638e-4 * t10091;
    let t10093 = t618 * t570;
    (t10090, t10092, t10093)
}
