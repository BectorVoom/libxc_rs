//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 877/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk877<F: Float>(t42177: F, t42180: F, t290: F, t9595: F, t1664: F, t2231: F, t42201: F, t42204: F, t42206: F, t42217: F, t942: F, t9639: F) -> (F, F, F, F, F, F, F, F, F) {
    let t44399 = F::cast_from(0.39726959900411316772e-4_f64) * t42177;
    let t44400 = F::cast_from(0.39726959900411316772e-4_f64) * t42180;
    let t44405 = t290 * t9595;
    let t44410 = t1664 * t2231;
    let t44423 = F::cast_from(0.1454648621559751559e0_f64) * t42201;
    let t44424 = F::cast_from(0.35754263910370185096e-3_f64) * t42204;
    let t44425 = F::cast_from(0.23836175940246790064e-3_f64) * t42206;
    let t44428 = F::cast_from(0.11918087970123395032e-3_f64) * t42217;
    let t44431 = F::new(0.4726e1) * t942 * t9639;
    (t44399, t44400, t44405, t44410, t44423, t44424, t44425, t44428, t44431)
}
