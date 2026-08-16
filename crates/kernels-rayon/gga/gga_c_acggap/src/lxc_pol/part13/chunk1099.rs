//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1099/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1099(t30916: f64, t30918: f64, t30921: f64, t35040: f64, t35042: f64, t35043: f64, t35047: f64, t35052: f64, t35055: f64, t35059: f64, t35062: f64, t35065: f64, t35068: f64, t35071: f64, t35073: f64, t35075: f64, t35076: f64, t35080: f64) -> f64 {
    let t35082 = 0.85748036236139473944e-3_f64 * t30916 + t35040 + t35042 - 35.0_f64 / 216.0_f64 * t35043 - 0.10718504529517434243e-3_f64 * t35047 - t35052 - 0.7862023072401038017e-3_f64 * t35055 + 0.47172138434406228102e-3_f64 * t30918 - t35059 / 16.0_f64 - t35062 / 16.0_f64 - 0.22921875e-1_f64 * t35065 - 0.4584375e-1_f64 * t35068 - t35071 - t35073 - t35075 - t30921 - 77.0_f64 / 576.0_f64 * t35076 - 0.38203125e-2_f64 * t35080;
    t35082
}
