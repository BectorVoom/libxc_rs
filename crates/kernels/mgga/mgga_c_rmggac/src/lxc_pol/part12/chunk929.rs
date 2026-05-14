//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 929/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk929<F: Float>(t2191: F, t8582: F, t2868: F, t7855: F, t2057: F, t26370: F, t9000: F, t9128: F, t7244: F, t9165: F, t27124: F, t3351: F, t3352: F, t515: F, t2286: F, t36542: F) -> (F, F, F, F, F, F, F) {
    let t41971 = t2191 * t8582;
    let t41973 = t2868 * t7855;
    let t41975 = t26370 * t2057;
    let t41977 = t9128 * t9000;
    let t41978 = 0.15965655602485078085e0 * t41977;
    let t41979 = t7244 * t9165;
    let t41980 = 0.19863479950205658386e-4 * t41979;
    let t41983 = t3351 * t3352 * t515 * t27124;
    let t41985 = t36542 * t2286;
    (t41971, t41973, t41975, t41978, t41980, t41983, t41985)
}
