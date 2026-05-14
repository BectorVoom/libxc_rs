//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 658/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk658<F: Float>(t10153: F, t10193: F, t82: F, t72: F, t10148: F, t884: F, t2405: F, t534: F, t2292: F, t5928: F, t2376: F, t2868: F, t8500: F, t8692: F, t8698: F, t9037: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t10194 = t10153 + t10193;
    let t10195 = t82 * t10194;
    let t10196 = t72 * t10195;
    let t10197 = t884 * t10148;
    let t10198 = 0.59871208509319042821e-1 * t10197;
    let t10199 = t534 * t2405;
    let t10200 = t72 * t10199;
    let t10201 = 2.0 * t10200;
    let t10203 = t5928 * t2292;
    let t10204 = 0.79828278012425390428e-1 * t10203;
    let t10205 = t2868 * t2376;
    let t10206 = 0.11974241701863808564e0 * t10205;
    let t10280 = 0.39726959900411316772e-4 * t8500;
    let t10357 = 0.39726959900411316772e-4 * t8692;
    let t10360 = 0.39726959900411316772e-4 * t8698;
    let t10383 = 0.49658699875514145965e-4 * t9037;
    (t10194, t10195, t10196, t10198, t10199, t10201, t10204, t10206, t10280, t10357, t10360, t10383)
}
