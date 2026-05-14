//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 784/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk784<F: Float>(t42023: F, t42026: F, t42044: F, t42086: F, t42101: F, t40803: F, t40831: F, t40907: F, t40918: F, t40970: F, t40976: F, t41041: F, t41057: F, t41114: F, t41128: F, t41438: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t43978 = 0.162600798888400151e-2 * t42023;
    let t43979 = 0.162600798888400151e-2 * t42026;
    let t43987 = 0.11918087970123395032e-3 * t42044;
    let t44004 = 0.39726959900411316772e-4 * t42086;
    let t44008 = 0.11918087970123395032e-3 * t42101;
    let t44029 = 0.3193131120497015617e0 * t40803;
    let t44035 = 0.3193131120497015617e0 * t40831;
    let t44070 = 0.21819729323396273384e0 * t40907;
    let t44075 = 0.10909864661698136692e0 * t40918;
    let t44093 = 0.10909864661698136692e0 * t40970;
    let t44095 = 0.1454648621559751559e0 * t40976;
    let t44110 = 0.36366215538993788974e-1 * t41041;
    let t44114 = 0.10909864661698136692e0 * t41057;
    let t44143 = 0.15965655602485078085e0 * t41114;
    let t44145 = 0.3193131120497015617e0 * t41128;
    let t44169 = 0.3193131120497015617e0 * t41438;
    (t43978, t43979, t43987, t44004, t44008, t44029, t44035, t44070, t44075, t44093, t44095, t44110, t44114, t44143, t44145, t44169)
}
