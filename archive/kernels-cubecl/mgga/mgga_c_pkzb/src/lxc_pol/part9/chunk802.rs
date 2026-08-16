//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 802/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk802<F: Float>(t2030: F, t5728: F, t5727: F, t758: F, t1854: F, t659: F, t1857: F, t1856: F, t683: F) -> (F, F, F, F, F, F) {
    let t5729 = t5728 * t2030;
    let t5730 = t5727 * t5729;
    let t5731 = t758 * t5730;
    let t5734 = t659 * t1854;
    let t5736 = F::cast_from(6.0_f64) * t5734 * t1857;
    let t5737 = t1856 * t683;
    (t5729, t5730, t5731, t5734, t5736, t5737)
}
