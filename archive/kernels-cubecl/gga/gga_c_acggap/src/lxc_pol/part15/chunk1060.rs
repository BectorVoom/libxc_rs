//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1060/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1060<F: Float>(t36017: F, t36030: F, t36039: F, t36041: F, t36065: F, t36081: F, t36085: F, t36087: F, t36089: F, t36096: F, t36125: F, t36131: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37830 = F::cast_from(0.68598428988911579156e-2_f64) * t36017;
    let t37833 = F::cast_from(0.62896184579208304138e-3_f64) * t36030;
    let t37836 = F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t36039;
    let t37837 = F::cast_from(7.0_f64) / F::cast_from(36.0_f64) * t36041;
    let t37848 = F::cast_from(11.0_f64) / F::cast_from(144.0_f64) * t36065;
    let t37857 = F::cast_from(0.12579236915841660828e-2_f64) * t36081;
    let t37859 = F::cast_from(0.21437009059034868486e-2_f64) * t36085;
    let t37860 = F::cast_from(0.85748036236139473944e-3_f64) * t36087;
    let t37861 = F::cast_from(0.42874018118069736972e-3_f64) * t36089;
    let t37864 = F::cast_from(0.62896184579208304138e-3_f64) * t36096;
    let t37872 = F::cast_from(0.32012600194825403606e-1_f64) * t36125;
    let t37875 = F::cast_from(0.85748036236139473944e-3_f64) * t36131;
    (t37830, t37833, t37836, t37837, t37848, t37857, t37859, t37860, t37861, t37864, t37872, t37875)
}
