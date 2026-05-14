//! MGGA_C_RMGGAC lxc pol — lxc_pol part 14 (v4rho3sigma_5) CSE chunk 511/952 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part14_v4rho3sigma_5_chunk511<F: Float>(t1973: F, t7255: F, t236: F, t4564: F, t1971: F, t1970: F, t325: F, t874: F) -> (F, F, F, F) {
    let t7256 = t7255 * t1973;
    let t7257 = 0.85129199786595678796e-5 * t7256;
    let t7258 = t236 * t4564;
    let t7259 = t1971 * t7258;
    let t7260 = t1970 * t7259;
    let t7261 = 0.42564599893297839398e-5 * t7260;
    let t7262 = t325 * t874;
    (t7257, t7259, t7261, t7262)
}
