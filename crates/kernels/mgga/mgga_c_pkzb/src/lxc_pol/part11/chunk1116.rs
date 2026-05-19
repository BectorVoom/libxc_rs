//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1116/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1116<F: Float>(t22233: F, t2239: F, t3030: F, t1171: F, t6198: F, t2317: F, t3113: F, t1201: F, t6230: F, t2278: F, t3080: F, t1189: F, t6287: F) -> (F, F, F, F, F, F, F, F) {
    let t22706 = F::cast_from(0.68493333333333333332e-1_f64) * t22233;
    let t22716 = F::cast_from(0.71233333333333333332e-1_f64) * t22233;
    let t22722 = t3030 * t2239;
    let t22727 = t1171 * t6198;
    let t22745 = t3113 * t2317;
    let t22750 = t1201 * t6230;
    let t22762 = t3080 * t2278;
    let t22767 = t1189 * t6287;
    (t22706, t22716, t22722, t22727, t22745, t22750, t22762, t22767)
}
