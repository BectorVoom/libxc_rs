//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1134/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1134<F: Float>(t1008: F, t14554: F, t167: F, t27819: F, t1020: F, t13284: F, t26760: F, t13288: F, t2842: F, t1092: F, t1121: F, t27763: F, t5042: F, t26671: F, t4806: F, t4548: F) -> (F, F, F, F, F, F) {
    let t95721 = t14554 * t27819 * t167 * t1008;
    let t95727 = t1020 * t26760 * t13284;
    let t95730 = t2842 * t26760 * t13288;
    let t95736 = t1092 * t27763 * t5042 * t1121;
    let t95739 = t1020 * t26671 * t4806;
    let t95742 = t1020 * t26671 * t4548;
    (t95721, t95727, t95730, t95736, t95739, t95742)
}
