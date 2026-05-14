//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 724/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk724<F: Float>(t2005: F, t5477: F, t2019: F, t657: F, t2023: F, t5509: F, t1586: F, t163: F, t397: F, t2024: F, t782: F, t4419: F, t5516: F, t5510: F, t2020: F, t4597: F) -> (F, F, F, F, F, F, F) {
    let t12251 = t2005 * t5477;
    let t12253 = t2019 * t2019;
    let t12254 = 1.0 / t12253;
    let t12255 = t657 * t12254;
    let t12256 = t5509 * t2023;
    let t12257 = t12255 * t12256;
    let t12258 = t1586 * t12257;
    let t12261 = t397 * t163;
    let t12262 = t12261 * t2024;
    let t12263 = t782 * t12262;
    let t12265 = t4419 * t5516;
    let t12266 = t782 * t12265;
    let t12268 = t4419 * t5510;
    let t12269 = t782 * t12268;
    let t12271 = t2020 * t4597;
    (t12251, t12258, t12261, t12263, t12266, t12269, t12271)
}
