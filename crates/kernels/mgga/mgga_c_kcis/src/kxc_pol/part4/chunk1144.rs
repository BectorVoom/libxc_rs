//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1144/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1144<F: Float>(t14522: F, t417: F, t2872: F, t4936: F, t1699: F, t9916: F, t991: F, t14486: F, t14489: F, t14493: F, t14498: F, t14502: F, t14513: F, t14518: F, t1700: F, t4940: F, t4944: F, t4948: F, t9903: F) -> F {
    let t14523 = t417 * t14522;
    let t14527 = t2872 * t4936 / F::cast_from(162.0_f64);
    let t14528 = t9916 * t1699;
    let t14529 = t991 * t14528;
    let t14531 = -t991 * t14486 / F::cast_from(144.0_f64) + t991 * t14489 / F::cast_from(216.0_f64) + F::cast_from(7.0_f64) / F::cast_from(648.0_f64) * t991 * t14493 + t991 * t14498 / F::cast_from(54.0_f64) - t991 * t14502 / F::cast_from(288.0_f64) + t2872 * t4944 / F::cast_from(54.0_f64) + t2872 * t4948 / F::cast_from(27.0_f64) - F::cast_from(2.0_f64) / F::cast_from(81.0_f64) * t2872 * t4940 + t991 * t14513 / F::cast_from(24.0_f64) + t14518 + F::cast_from(11.0_f64) / F::cast_from(324.0_f64) * t9903 * t1700 - t991 * t14523 / F::cast_from(16.0_f64) - t14527 - t14529 / F::cast_from(1296.0_f64);
    t14531
}
