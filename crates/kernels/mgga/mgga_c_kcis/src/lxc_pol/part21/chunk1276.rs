//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1276/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1276<F: Float>(t15092: F, t2189: F, t3330: F, t1820: F, t26950: F, t1203: F, t28071: F, t14654: F, t283: F, t990: F, t26753: F, t27845: F, t4994: F) -> (F, F, F, F, F) {
    let t95514 = F::cast_from(2.0_f64) * t3330 * t2189 * t15092;
    let t95517 = F::cast_from(2.0_f64) * t3330 * t26950 * t1820;
    let t95520 = F::cast_from(4.0_f64) * t3330 * t28071 * t1203;
    let t95524 = t14654 * t283 * t990;
    let t95532 = t4994 * t26753 * t27845;
    (t95514, t95517, t95520, t95524, t95532)
}
