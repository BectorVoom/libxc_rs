//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1119/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1119<F: Float>(t22988: F, t3174: F, t3176: F, t487: F, t1228: F, t300: F, t19107: F, t22971: F, t19116: F, t54: F, t8253: F, t1167: F, t179: F, t19150: F, t404: F) -> (F, F, F, F, F, F, F) {
    let t22989 = F::cast_from(0.28582678745379824648e-3_f64) * t22988;
    let t23007 = t3174 * t487 * t3176;
    let t23008 = t23007 / F::new(72.0);
    let t23054 = t300 * t1228;
    let t23075 = t19107 * t22971;
    let t23081 = t19116 * t22971;
    let t23213 = t54 * t8253;
    let t23272 = t404 * t179 * t19150 * t1167;
    (t22989, t23008, t23054, t23075, t23081, t23213, t23272)
}
