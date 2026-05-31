//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 1248/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk1248<F: Float>(t3262: F, t3472: F, t42919: F, t11523: F, t12033: F, t1115: F, t2530: F, t3270: F, t3579: F, t11338: F, t12567: F, t3465: F, t43717: F) -> (F, F, F, F, F) {
    let t44524 = F::cast_from(15.0_f64) / F::cast_from(8.0_f64) * t3262 * t3472 * t42919;
    let t44526 = t11523 * t12033 / F::cast_from(2.0_f64);
    let t44530 = t3579 * t3270 * t1115 * t2530 / F::cast_from(2.0_f64);
    let t44532 = t12567 * t11338 / F::cast_from(4.0_f64);
    let t44535 = F::cast_from(3.0_f64) / F::cast_from(4.0_f64) * t3262 * t3465 * t43717;
    (t44524, t44526, t44530, t44532, t44535)
}
