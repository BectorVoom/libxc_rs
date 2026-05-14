//! LDA_C_PW_ERF lxc pol — lxc_pol part 4 (v4rho4_2) CSE chunk 692/1374 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part4_v4rho4_2_chunk692<F: Float>(t1627: F, t1926: F, t20: F, t2259: F, t1639: F, t3707: F, t3736: F, t3749: F, t3760: F, t3764: F, t3785: F, t3789: F, t4185: F, t4188: F, t4190: F, t4193: F, t4198: F, t4201: F, t4202: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4544 = t1926 * t1627;
    let t4546 = t2259 * t20;
    let t4547 = t4546 * t1639;
    let t4549 = 16.0 / 135.0 * t3707;
    let t4550 = 16.0 / 135.0 * t3736;
    let t4551 = 16.0 / 135.0 * t3749;
    let t4552 = 16.0 / 135.0 * t3760;
    let t4553 = 16.0 / 405.0 * t3764;
    let t4554 = 16.0 / 405.0 * t3785;
    let t4555 = 16.0 / 45.0 * t3789;
    let t4560 = 0.07214027574909895 * t4544 + 0.011181742741110338 * t4547 + t4549 - t4550 - t4551 + t4552 - t4553 - t4554 - t4555 - t4185 + 0.10821041362364843 * t4188 + 0.4328416544945937 * t4190 + 0.022363485482220676 * t4193 + t4198 + t4201 + 0.1442805514981979 * t4202;
    (t4544, t4546, t4547, t4549, t4550, t4551, t4552, t4553, t4554, t4555, t4560)
}
