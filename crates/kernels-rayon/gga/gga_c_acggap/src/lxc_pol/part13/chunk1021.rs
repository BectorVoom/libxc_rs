//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1021/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1021(t30265: f64, t34028: f64, t34030: f64, t34032: f64, t34033: f64, t34036: f64, t34038: f64, t34039: f64, t34041: f64, t34043: f64, t34048: f64, t34053: f64, t34054: f64, t34056: f64, t34058: f64, t34059: f64, t34063: f64, t34068: f64) -> f64 {
    let t34070 = t34028 + t34030 - t34032 - 0.10718504529517434243e-3_f64 * t34033 - t34036 - t34038 - 0.14291339372689912324e-3_f64 * t34039 - 0.85748036236139473944e-3_f64 * t34041 + 0.19055119163586549766e-2_f64 * t34043 - 0.53592522647587171215e-3_f64 * t34048 - t34053 - 0.13208198761633743869e-1_f64 * t34054 - 0.7145669686344956162e-3_f64 * t34056 - t34058 + 0.62896184579208304136e-3_f64 * t34059 - t34063 / 384.0_f64 - 0.41930789719472202756e-3_f64 * t30265 - 0.42874018118069736972e-3_f64 * t34068;
    t34070
}
