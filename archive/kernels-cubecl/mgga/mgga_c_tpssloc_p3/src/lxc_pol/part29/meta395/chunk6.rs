//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 1617/2357 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk1617<F: Float>(t3377: F, t4861: F, t14722: F, t14704: F, t11137: F, t11139: F, t11141: F, t11143: F, t11444: F, t14702: F, t14708: F, t14720: F, t14728: F, t14733: F, t14738: F, t14742: F, t14746: F, t14751: F, t14755: F) -> (F, F) {
    let t15185 = t4861 * t3377;
    let t15194 = F::cast_from(0.2283111111111111111e-1_f64) * t14722;
    let t15195 = F::cast_from(0.11415555555555555555e-1_f64) * t14704;
    let t15204 = -t11444 + F::cast_from(0.1522074074074074074e-1_f64) * t11137 + F::cast_from(0.38051851851851851851e-2_f64) * t11139 - F::cast_from(0.11415555555555555555e-1_f64) * t11141 - F::cast_from(0.57077777777777777777e-2_f64) * t11143 + F::cast_from(0.76103703703703703702e-2_f64) * t14702 + F::cast_from(0.76103703703703703701e-2_f64) * t14720 - t15194 - t15195 + F::cast_from(0.19025925925925925925e-1_f64) * t14728 - F::cast_from(0.68493333333333333331e-1_f64) * t14733 - F::cast_from(0.2283111111111111111e-1_f64) * t14738 - F::cast_from(0.11415555555555555555e-1_f64) * t14742 + F::cast_from(0.10274e0_f64) * t14746 + F::cast_from(0.68493333333333333332e-1_f64) * t14751 + F::cast_from(0.34246666666666666666e-1_f64) * t14755 + F::cast_from(0.17123333333333333333e-1_f64) * t14708;
    (t15185, t15204)
}
