//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 760/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk760<F: Float>(t2186: F, t8587: F, t9000: F, t9128: F, t7244: F, t9165: F, t2160: F, t638: F, t8850: F, t8854: F, t5055: F, t7769: F, t1341: F, t575: F, t7310: F, t8427: F) -> (F, F, F, F, F, F, F, F) {
    let t41960 = t2186 * t8587;
    let t41977 = t9128 * t9000;
    let t41978 = 0.15965655602485078085e0 * t41977;
    let t41979 = t7244 * t9165;
    let t41980 = 0.19863479950205658386e-4 * t41979;
    let t42023 = t638 * t2160 * t8850;
    let t42024 = 0.81300399444200075504e-3 * t42023;
    let t42026 = t638 * t2160 * t8854;
    let t42027 = 0.81300399444200075504e-3 * t42026;
    let t42034 = t5055 * t7769;
    let t42035 = 0.23948483403727617128e0 * t42034;
    let t42042 = t638 * t7310 * t575 * t1341;
    let t42044 = t7244 * t8427;
    (t41960, t41978, t41980, t42024, t42027, t42035, t42042, t42044)
}
