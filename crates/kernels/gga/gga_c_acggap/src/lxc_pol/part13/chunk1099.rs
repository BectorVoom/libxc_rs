//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1099/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1099<F: Float>(t30916: F, t30918: F, t30921: F, t35040: F, t35042: F, t35043: F, t35047: F, t35052: F, t35055: F, t35059: F, t35062: F, t35065: F, t35068: F, t35071: F, t35073: F, t35075: F, t35076: F, t35080: F) -> F {
    let t35082 = F::cast_from(0.85748036236139473944e-3_f64) * t30916 + t35040 + t35042 - F::new(35.0) / F::new(216.0) * t35043 - F::cast_from(0.10718504529517434243e-3_f64) * t35047 - t35052 - F::cast_from(0.7862023072401038017e-3_f64) * t35055 + F::cast_from(0.47172138434406228102e-3_f64) * t30918 - t35059 / F::new(16.0) - t35062 / F::new(16.0) - F::new(0.22921875e-1) * t35065 - F::new(0.4584375e-1) * t35068 - t35071 - t35073 - t35075 - t30921 - F::new(77.0) / F::new(576.0) * t35076 - F::new(0.38203125e-2) * t35080;
    t35082
}
