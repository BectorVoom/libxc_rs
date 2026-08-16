//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 351/1089 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk351<F: Float>(t2085: F, t648: F, t2028: F, t305: F, t2031: F, t326: F, t118: F, t2025: F, t2069: F, t793: F, t2074: F, t797: F) -> (F, F, F, F, F, F) {
    let t2086 = t648 * t2085;
    let t2087 = F::cast_from(0.90915538847484472429e-2_f64) * t2086;
    let t2088 = t305 * t2028;
    let t2090 = t326 * t2031;
    let t2092 = t118 * t2025;
    let t2094 = t793 * t2069;
    let t2096 = t797 * t2074;
    (t2087, t2088, t2090, t2092, t2094, t2096)
}
