//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1210/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1210<F: Float>(t34009: F, t34011: F, t34023: F, t34029: F, t34033: F, t34039: F, t36893: F, t36900: F, t36908: F, t36910: F, t36912: F, t36913: F, t38909: F, t38912: F, t38914: F, t38916: F, t38920: F, t38925: F) -> F {
    let t41404 = t36893 + F::cast_from(0.18868855373762491241e-2_f64) * t38909 + F::cast_from(0.85748036236139473944e-3_f64) * t34009 - F::cast_from(0.16772315887788881104e-1_f64) * t34011 + t36900 - F::cast_from(0.17149607247227894789e-2_f64) * t38912 + F::cast_from(0.56606566121287473725e-1_f64) * t34023 + t36908 + F::cast_from(0.25724410870841842183e-2_f64) * t34029 - t36910 - F::cast_from(0.42874018118069736972e-3_f64) * t34033 - t36912 - t36913 - F::cast_from(0.57165357490759649296e-3_f64) * t34039 - F::cast_from(0.10718504529517434243e-2_f64) * t38914 + F::cast_from(0.40015750243531754507e-2_f64) * t38916 - F::cast_from(0.10718504529517434243e-2_f64) * t38920 - F::cast_from(0.10718504529517434243e-2_f64) * t38925;
    t41404
}
