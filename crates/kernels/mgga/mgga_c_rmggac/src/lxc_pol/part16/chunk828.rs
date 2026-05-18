//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 828/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk828<F: Float>(t26: F, t7834: F, t797: F, t838: F, t40331: F, t793: F, t558: F, t7817: F, t305: F, t38381: F, t262: F, t40802: F) -> (F, F, F, F, F, F, F) {
    let t40927 = t7834 * t26;
    let t40928 = t797 * t40927;
    let t40932 = t838 * t40927;
    let t40944 = t793 * t40331;
    let t40948 = t7817 * t558;
    let t40949 = t797 * t40948;
    let t40951 = t305 * t38381;
    let t40965 = t262 * t40802;
    (t40928, t40932, t40944, t40948, t40949, t40951, t40965)
}
