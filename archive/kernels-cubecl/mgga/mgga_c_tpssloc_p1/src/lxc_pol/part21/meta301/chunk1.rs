//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 1639/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk1639<F: Float>(t11135: F, t10292: F, t281: F, t415: F, t1114: F, t2403: F) -> (F, F, F, F) {
    let t11195 = F::cast_from(0.93011851851851851854e0_f64) * t11135;
    let t11203 = t281 * t10292 * t415;
    let t11204 = F::cast_from(0.36514074074074074075e0_f64) * t11203;
    let t11211 = t2403 * t1114;
    (t11195, t11203, t11204, t11211)
}
