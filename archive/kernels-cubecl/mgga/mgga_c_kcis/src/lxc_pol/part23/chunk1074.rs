//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1074/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1074<F: Float>(t27636: F, t4457: F, t6176: F, t3805: F, t7979: F, t1600: F, t27389: F, t27392: F, t27400: F, t27420: F, t27425: F, t27429: F, t27598: F, t27626: F, t7968: F, t7978: F) -> (F, F, F, F, F) {
    let t27637 = t27636 * t4457;
    let t27638 = t6176 * t27637;
    let t27641 = t7979 * t3805;
    let t27642 = t1600 * t27641;
    let t27645 = -F::cast_from(0.23214722222222222222e-2_f64) * t27389 - F::cast_from(0.13913205078125e-3_f64) * t7968 * t27598 - F::cast_from(0.15445601851851851852e-3_f64) * t7978 * t27626 - F::cast_from(0.17411041666666666666e-2_f64) * t27392 + F::cast_from(0.11607361111111111111e-2_f64) * t27400 - F::cast_from(0.38691203703703703703e-3_f64) * t27420 - F::cast_from(0.23214722222222222222e-2_f64) * t27425 - F::cast_from(0.23214722222222222222e-2_f64) * t27429 - F::cast_from(0.69505208333333333334e-3_f64) * t7978 * t27598 - F::cast_from(0.69505208333333333334e-3_f64) * t7978 * t27638 - F::cast_from(0.11584201388888888889e-3_f64) * t7978 * t27642;
    (t27637, t27638, t27641, t27642, t27645)
}
