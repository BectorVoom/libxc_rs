//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1021/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1021<F: Float>(t30265: F, t34028: F, t34030: F, t34032: F, t34033: F, t34036: F, t34038: F, t34039: F, t34041: F, t34043: F, t34048: F, t34053: F, t34054: F, t34056: F, t34058: F, t34059: F, t34063: F, t34068: F) -> F {
    let t34070 = t34028 + t34030 - t34032 - F::cast_from(0.10718504529517434243e-3_f64) * t34033 - t34036 - t34038 - F::cast_from(0.14291339372689912324e-3_f64) * t34039 - F::cast_from(0.85748036236139473944e-3_f64) * t34041 + F::cast_from(0.19055119163586549766e-2_f64) * t34043 - F::cast_from(0.53592522647587171215e-3_f64) * t34048 - t34053 - F::cast_from(0.13208198761633743869e-1_f64) * t34054 - F::cast_from(0.7145669686344956162e-3_f64) * t34056 - t34058 + F::cast_from(0.62896184579208304136e-3_f64) * t34059 - t34063 / F::cast_from(384.0_f64) - F::cast_from(0.41930789719472202756e-3_f64) * t30265 - F::cast_from(0.42874018118069736972e-3_f64) * t34068;
    t34070
}
