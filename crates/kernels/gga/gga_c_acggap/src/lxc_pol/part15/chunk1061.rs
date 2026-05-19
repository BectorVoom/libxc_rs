//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1061/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1061<F: Float>(t36133: F, t36151: F, t36156: F, t36162: F, t36175: F, t36177: F, t36194: F, t36198: F, t36205: F, t36207: F, t36210: F, t36214: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37876 = F::cast_from(0.85748036236139473944e-3_f64) * t36133;
    let t37888 = F::new(7.0) / F::new(72.0) * t36151;
    let t37892 = F::cast_from(0.12579236915841660828e-2_f64) * t36156;
    let t37894 = F::cast_from(0.85748036236139473944e-3_f64) * t36162;
    let t37898 = F::cast_from(0.18868855373762491241e-2_f64) * t36175;
    let t37899 = F::cast_from(0.68598428988911579156e-2_f64) * t36177;
    let t37904 = F::cast_from(0.62896184579208304136e-2_f64) * t36194;
    let t37905 = F::cast_from(0.94344276868812456204e-2_f64) * t36198;
    let t37907 = F::new(0.36675e0) * t36205;
    let t37908 = F::new(0.183375e0) * t36207;
    let t37909 = F::new(0.183375e0) * t36210;
    let t37910 = F::new(0.183375e0) * t36214;
    (t37876, t37888, t37892, t37894, t37898, t37899, t37904, t37905, t37907, t37908, t37909, t37910)
}
