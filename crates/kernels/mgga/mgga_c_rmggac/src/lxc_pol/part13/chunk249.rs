//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 249/988 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk249<F: Float>(t1156: F, t197: F, t1144: F, t1149: F, t1152: F, t198: F, t446: F, t454: F, t998: F, t201: F, t457: F, t461: F, t495: F, t225: F, t226: F) -> (F, F, F, F, F, F, F, F) {
    let t1157 = t197 * t1156;
    let t1162 = -0.32163648644302209643e2 * t1149 * t198 + 0.19298189186581325786e3 * t1152 * t446 - 0.38596378373162651572e3 * t1157 * t1144 + 0.96490945932906628929e2 * t454 * t998;
    let t1163 = t1162 * t201;
    let t1165 = t457 * t457;
    let t1166 = t1165 * t201;
    let t1168 = t461 * t495;
    let t1171 = t225 * t225;
    let t1173 = 1.0 / t226 / t1171;
    (t1157, t1162, t1163, t1165, t1166, t1168, t1171, t1173)
}
