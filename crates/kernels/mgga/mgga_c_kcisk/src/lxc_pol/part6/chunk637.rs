//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 637/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk637<F: Float>(t2019: F, t657: F, t163: F, t397: F, t2020: F, t4597: F, t2040: F, t798: F, t15: F, t944: F, t1014: F, t142: F, t3088: F, t5: F, t119: F, t955: F) -> (F, F, F, F, F, F, F) {
    let t12253 = t2019 * t2019;
    let t12254 = 1.0 / t12253;
    let t12255 = t657 * t12254;
    let t12261 = t397 * t163;
    let t12271 = t2020 * t4597;
    let t12350 = t2040 * t2040;
    let t12351 = 1.0 / t12350;
    let t12352 = t798 * t12351;
    let t12407 = t15 * t944;
    let t12408 = t1014 * t12407;
    let t12410 = t5 * t142 * t3088;
    let t12414 = t5 * t119 * t955;
    (t12255, t12261, t12271, t12352, t12408, t12410, t12414)
}
