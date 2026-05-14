//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 804/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk804<F: Float>(t1652: F, t2392: F, t262: F, t8620: F, t8577: F, t9165: F, t1971: F, t236: F, t36489: F, t495: F, t6108: F, t16503: F, t22971: F, t552: F, t8425: F, t14243: F, t8430: F) -> (F, F, F, F, F, F, F) {
    let t45166 = t2392 * t1652;
    let t45167 = t262 * t45166;
    let t45168 = t8620 * t45167;
    let t45170 = t8577 * t9165;
    let t45175 = t36489 * t1971 * t236 * t6108 * t495;
    let t45179 = t16503 * t22971 * t552 * t8425;
    let t45183 = t16503 * t14243 * t552 * t8430;
    (t45166, t45167, t45168, t45170, t45175, t45179, t45183)
}
