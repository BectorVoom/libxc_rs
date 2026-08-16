//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1274/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1274<F: Float>(t3330: F, t5189: F, t7766: F, t27999: F, t33853: F, t14665: F, t1820: F, t93243: F, t10498: F, t1203: F, t28005: F, t14683: F, t26871: F) -> (F, F, F, F, F, F) {
    let t95498 = F::cast_from(4.0_f64) * t3330 * t7766 * t5189;
    let t95500 = F::cast_from(12.0_f64) * t33853 * t27999;
    let t95502 = F::cast_from(2.0_f64) * t14665 * t7766;
    let t95503 = t93243 * t1820;
    let t95506 = F::cast_from(12.0_f64) * t10498 * t28005 * t1203;
    let t95508 = F::cast_from(2.0_f64) * t26871 * t14683;
    (t95498, t95500, t95502, t95503, t95506, t95508)
}
