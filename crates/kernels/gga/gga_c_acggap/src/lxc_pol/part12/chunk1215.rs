//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1215/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1215<F: Float>(t36151: F, t36156: F, t36162: F, t36175: F, t36177: F, t31759: F, t31761: F, t31763: F, t31774: F, t31782: F, t31790: F, t36147: F, t36149: F, t36160: F, t36165: F, t36169: F, t36173: F, t36181: F) -> F {
    let t37888 = F::new(7.0) / F::new(72.0) * t36151;
    let t37892 = F::cast_from(0.12579236915841660828e-2_f64) * t36156;
    let t37894 = F::cast_from(0.85748036236139473944e-3_f64) * t36162;
    let t37898 = F::cast_from(0.18868855373762491241e-2_f64) * t36175;
    let t37899 = F::cast_from(0.68598428988911579156e-2_f64) * t36177;
    let t37901 = -F::cast_from(0.7145669686344956162e-3_f64) * t31759 - F::cast_from(0.85748036236139473944e-3_f64) * t31761 - F::cast_from(0.42874018118069736972e-3_f64) * t31763 + t36147 / F::new(8.0) + t36149 / F::new(24.0) + t37888 + F::new(0.3361875e0) * t31774 + F::new(0.16809375e0) * t31782 - F::new(0.1120625e0) * t31790 - t37892 - F::cast_from(0.62896184579208304138e-3_f64) * t36160 + t37894 + F::cast_from(0.85748036236139473944e-3_f64) * t36165 + F::cast_from(0.85748036236139473944e-3_f64) * t36169 + F::cast_from(0.42874018118069736972e-3_f64) * t36173 - t37898 - t37899 - F::cast_from(0.18868855373762491241e-2_f64) * t36181;
    t37901
}
