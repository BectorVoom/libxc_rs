//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1002/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1002<F: Float>(t9000: F, t9128: F, t7244: F, t9165: F, t27124: F, t3351: F, t3352: F, t515: F, t2286: F, t36542: F, t1979: F, t1982: F, t458: F, t8601: F) -> (F, F, F, F, F) {
    let t41977 = t9128 * t9000;
    let t41979 = t7244 * t9165;
    let t41983 = t3351 * t3352 * t515 * t27124;
    let t41985 = t36542 * t2286;
    let t41989 = t8601 * t458 * t1979 * t1982;
    (t41977, t41979, t41983, t41985, t41989)
}
