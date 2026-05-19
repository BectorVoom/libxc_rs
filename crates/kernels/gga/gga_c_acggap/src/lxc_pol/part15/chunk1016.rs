//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1016/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1016<F: Float>(t31505: F, t31530: F, t31532: F, t1967: F, t8502: F, t1998: F, t5089: F, t1451: F, t7605: F, t1423: F, t7736: F, t30318: F, t542: F) -> (F, F, F, F, F, F, F, F) {
    let t35713 = F::cast_from(0.18007087609589289529e-1_f64) * t31505;
    let t35718 = F::cast_from(0.34299214494455789578e-2_f64) * t31530;
    let t35719 = F::cast_from(0.34299214494455789578e-2_f64) * t31532;
    let t35722 = t1967 * t8502;
    let t35733 = t1998 * t5089;
    let t35736 = t7605 * t1451;
    let t35738 = t7736 * t1423;
    let t35740 = t30318 * t542;
    (t35713, t35718, t35719, t35722, t35733, t35736, t35738, t35740)
}
