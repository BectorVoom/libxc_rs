//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1104/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1104<F: Float>(t1267: F, t26975: F, t5329: F, t6842: F, t1020: F, t4801: F, t95664: F, t2861: F, t28992: F, t19741: F, t7718: F, t18509: F, t18443: F, t3203: F, t26753: F, t2842: F, t28911: F) -> (F, F, F, F, F, F, F) {
    let t100170 = t5329 * t26975 * t6842 * t1267;
    let t100174 = t1020 * t95664 * t4801;
    let t100179 = t2861 * t28992;
    let t100188 = t1020 * t7718 * t19741;
    let t100191 = t1020 * t7718 * t18509;
    let t100198 = t1020 * t7718 * t3203 * t18443;
    let t100201 = t2842 * t26753 * t28911;
    (t100170, t100174, t100179, t100188, t100191, t100198, t100201)
}
