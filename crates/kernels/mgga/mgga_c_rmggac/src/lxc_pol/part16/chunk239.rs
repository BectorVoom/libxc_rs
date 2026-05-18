//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 239/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk239<F: Float>(t1156: F, t197: F, t461: F, t495: F, t225: F, t226: F) -> (F, F, F, F) {
    let t1157 = t197 * t1156;
    let t1168 = t461 * t495;
    let t1171 = t225 * t225;
    let t1173 = F::new(1.0) / t226 / t1171;
    (t1157, t1168, t1171, t1173)
}
