//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 552/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk552<F: Float>(t118: F, t7409: F, t1986: F, t7408: F, t1993: F, t2185: F, t1997: F, t4179: F, t6: F, t220: F, t1001: F, t128: F) -> (F, F, F, F, F, F, F) {
    let t7410 = t118 * t7409;
    let t7411 = t1986 * t7410;
    let t7412 = t7408 * t7411;
    let t7413 = F::cast_from(0.11971293719990017331e-4_f64) * t7412;
    let t7414 = t1993 * t2185;
    let t7415 = t7414 * t1997;
    let t7417 = t6 * t4179;
    let t7418 = t220 * t7417;
    let t7419 = t128 * t1001;
    (t7411, t7413, t7414, t7415, t7417, t7418, t7419)
}
