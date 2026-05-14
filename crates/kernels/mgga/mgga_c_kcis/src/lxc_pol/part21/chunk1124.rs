//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1124/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1124<F: Float>(t15092: F, t2189: F, t3330: F, t1820: F, t26950: F, t1203: F, t28071: F, t14654: F, t283: F, t990: F, t26753: F, t27845: F, t4994: F, t33822: F, t27806: F, t2809: F, t42625: F) -> (F, F, F, F, F, F, F) {
    let t95514 = 2.0 * t3330 * t2189 * t15092;
    let t95517 = 2.0 * t3330 * t26950 * t1820;
    let t95520 = 4.0 * t3330 * t28071 * t1203;
    let t95524 = t14654 * t283 * t990;
    let t95532 = t4994 * t26753 * t27845;
    let t95535 = t33822 * t283 * t990;
    let t95537 = t27806 * t42625 * t2809;
    (t95514, t95517, t95520, t95524, t95532, t95535, t95537)
}
