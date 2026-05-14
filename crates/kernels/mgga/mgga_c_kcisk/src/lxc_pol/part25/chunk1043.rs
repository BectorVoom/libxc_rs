//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1043/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1043<F: Float>(t1683: F, t7506: F, t2609: F, t4781: F, t4762: F, t7509: F, t15993: F, t10570: F, t10572: F, t10574: F, t10576: F, t12002: F, t15989: F, t15991: F, t15996: F, t16001: F, t16006: F, t16011: F, t16015: F, t16019: F, t16024: F, t16028: F, t16032: F) -> (F, F, F, F) {
    let t18601 = t7506 * t1683;
    let t18604 = t2609 * t4781;
    let t18607 = t7509 * t4762;
    let t18616 = 0.2283111111111111111e-1 * t15993;
    let t18626 = -t12002 - 0.1522074074074074074e-1 * t10570 + 0.38051851851851851851e-2 * t10572 - 0.11415555555555555555e-1 * t10574 + 0.57077777777777777777e-2 * t10576 - 0.76103703703703703702e-2 * t15989 + 0.76103703703703703701e-2 * t15991 - t18616 - 0.1255711111111111111e0 * t15996 - 0.19025925925925925925e-1 * t16001 + 0.68493333333333333331e-1 * t16006 + 0.45662222222222222221e-1 * t16011 - 0.11415555555555555555e-1 * t16015 - 0.10274e0 * t16019 - 0.13698666666666666666e0 * t16024 + 0.34246666666666666666e-1 * t16028 + 0.34246666666666666666e-1 * t16032;
    (t18601, t18604, t18607, t18626)
}
