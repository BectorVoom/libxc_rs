//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1179/1322 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1179<F: Float>(t26854: F, t7696: F, t2173: F, t2174: F, t737: F, t61287: F, t7702: F) -> (F, F, F) {
    let t93569 = t7696 * t26854;
    let t93590 = F::cast_from(0.25742669753086419753e-3_f64) * t2173 * t737 * t2174;
    let t93592 = t7702 * t61287;
    (t93569, t93590, t93592)
}
