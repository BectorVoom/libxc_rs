//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 849/1110 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk849<F: Float>(t2186: F, t8587: F, t9000: F, t9128: F, t7244: F, t9165: F, t2160: F, t638: F, t8850: F, t8854: F, t5055: F, t7769: F) -> (F, F, F, F, F, F) {
    let t41960 = t2186 * t8587;
    let t41977 = t9128 * t9000;
    let t41978 = F::cast_from(0.15965655602485078085e0_f64) * t41977;
    let t41979 = t7244 * t9165;
    let t41980 = F::cast_from(0.19863479950205658386e-4_f64) * t41979;
    let t42023 = t638 * t2160 * t8850;
    let t42024 = F::cast_from(0.81300399444200075504e-3_f64) * t42023;
    let t42026 = t638 * t2160 * t8854;
    let t42027 = F::cast_from(0.81300399444200075504e-3_f64) * t42026;
    let t42034 = t5055 * t7769;
    (t41960, t41978, t41980, t42024, t42027, t42034)
}
