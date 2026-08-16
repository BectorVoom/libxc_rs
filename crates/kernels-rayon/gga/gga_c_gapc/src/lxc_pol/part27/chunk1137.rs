//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1137/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1137(t11781: f64, t3368: f64, t34036: f64, t34038: f64, t34043: f64, t34046: f64, t34048: f64, t34050: f64, t34052: f64, t34054: f64, t34056: f64, t34060: f64) -> f64 {
    let t34062 = t11781 * t3368;
    let t34064 = -0.58333107277199074076e-4_f64 * t34036 + 0.57970906942607043474e-5_f64 * t34038 - 0.3077456993052877797e-8_f64 * t34043 - 0.15387284965264388985e-8_f64 * t34046 + 0.99443481748595550042e-7_f64 * t34048 - 0.10316808205282028424e-4_f64 * t34050 + 0.1600868508130162607e-6_f64 * t34052 + 0.14302847739140993952e-5_f64 * t34054 + 0.70341874126922921073e-8_f64 * t34056 + 0.23286599093046454432e-9_f64 * t34060 + 0.24760339692676868218e-5_f64 * t34062;
    t34064
}
