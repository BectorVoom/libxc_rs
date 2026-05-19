//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1137/1310 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1137<F: Float>(t11781: F, t3368: F, t34036: F, t34038: F, t34043: F, t34046: F, t34048: F, t34050: F, t34052: F, t34054: F, t34056: F, t34060: F) -> F {
    let t34062 = t11781 * t3368;
    let t34064 = -F::cast_from(0.58333107277199074076e-4_f64) * t34036 + F::cast_from(0.57970906942607043474e-5_f64) * t34038 - F::cast_from(0.3077456993052877797e-8_f64) * t34043 - F::cast_from(0.15387284965264388985e-8_f64) * t34046 + F::cast_from(0.99443481748595550042e-7_f64) * t34048 - F::cast_from(0.10316808205282028424e-4_f64) * t34050 + F::cast_from(0.1600868508130162607e-6_f64) * t34052 + F::cast_from(0.14302847739140993952e-5_f64) * t34054 + F::cast_from(0.70341874126922921073e-8_f64) * t34056 + F::cast_from(0.23286599093046454432e-9_f64) * t34060 + F::cast_from(0.24760339692676868218e-5_f64) * t34062;
    t34064
}
