//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 992/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk992<F: Float>(t40928: F, t5149: F, t649: F, t40927: F, t838: F, t5268: F, t40756: F, t797: F, t1614: F, t664: F, t40331: F, t793: F) -> (F, F, F, F, F) {
    let t40930 = t40928 * t649 * t5149;
    let t40932 = t838 * t40927;
    let t40934 = t40932 * t649 * t5268;
    let t40938 = t797 * t40756;
    let t40940 = t664 * t1614;
    let t40944 = t793 * t40331;
    (t40930, t40934, t40938, t40940, t40944)
}
