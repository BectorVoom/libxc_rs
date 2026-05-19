//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1165/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1165<F: Float>(t34170: F, t34172: F, t34175: F, t34179: F, t34189: F, t34204: F, t34217: F, t30308: F, t30310: F, t30314: F, t30316: F, t30319: F, t34183: F, t34193: F, t34197: F, t34201: F, t34208: F, t34215: F) -> F {
    let t36967 = F::cast_from(0.21437009059034868486e-2_f64) * t34170;
    let t36968 = F::cast_from(0.13719685797782315831e-1_f64) * t34172;
    let t36969 = F::cast_from(0.21437009059034868486e-2_f64) * t34175;
    let t36970 = F::cast_from(0.20965394859736101378e-2_f64) * t34179;
    let t36972 = F::cast_from(0.12579236915841660827e-1_f64) * t34189;
    let t36976 = F::cast_from(0.16006300097412701803e-1_f64) * t34204;
    let t36984 = F::cast_from(0.12579236915841660828e-2_f64) * t34217;
    let t36985 = -t36967 + t36968 + t36969 - t36970 - F::cast_from(0.94344276868812456208e-2_f64) * t34183 - t36972 + F::cast_from(0.37737710747524982482e-2_f64) * t34193 + F::cast_from(0.31448092289604152068e-2_f64) * t34197 - F::cast_from(0.47172138434406228102e-2_f64) * t34201 - t36976 - F::cast_from(0.75475421495049964966e-2_f64) * t34208 - F::new(77.0) / F::new(144.0) * t30308 - F::new(77.0) / F::new(432.0) * t30310 - F::new(0.1528125e-1) * t30314 - F::cast_from(0.62896184579208304138e-3_f64) * t30316 + F::cast_from(0.32012600194825403606e-1_f64) * t30319 - F::cast_from(0.62896184579208304138e-3_f64) * t34215 - t36984;
    t36985
}
