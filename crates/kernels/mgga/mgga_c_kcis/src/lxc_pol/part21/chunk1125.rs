//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1125/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1125<F: Float>(t1092: F, t19575: F, t26760: F, t3219: F, t14650: F, t1008: F, t829: F, t4961: F, t93426: F, t1009: F, t14407: F, t2811: F, t44756: F, t4566: F, t26703: F, t26806: F, t27832: F, t44504: F, t7703: F, t7704: F, t93425: F, t93592: F, t95524: F, t95532: F, t95535: F, t95537: F, t9924: F) -> (F, F, F, F, F) {
    let t95542 = t1092 * t26760 * t19575 * t3219;
    let t95545 = t1092 * t26760 * t14650;
    let t95547 = t829 * t1008;
    let t95549 = t93426 * t4961 * t95547;
    let t95552 = t14407 * t1009;
    let t95557 = t44756 * t2811;
    let t95559 = t95557 * t4566 * t95547;
    let t95564 = 0.46336805555555555556e-3 * t27832 * t26703 + 0.61836467013888888889e-4 * t95524 * t26703 - 0.46336805555555555556e-3 * t7703 * t9924 * t7704 * t44504 + 0.66327777777777777776e-2 * t95532 + 0.49555782539766601562e-5 * t95535 * t95537 + 0.55273148148148148147e-3 * t95542 + 0.11054629629629629629e-2 * t95545 - 0.61836467013888888889e-4 * t93425 * t95549 + 0.61782407407407407408e-3 * t93592 * t95552 * t4566 * t26806 + 0.61782407407407407408e-3 * t93592 * t95559 + 0.82448622685185185186e-4 * t93425 * t95559;
    (t95542, t95545, t95547, t95549, t95564)
}
