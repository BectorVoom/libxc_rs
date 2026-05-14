//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 941/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk941<F: Float>(t36017: F, t36030: F, t36039: F, t36041: F, t36065: F, t36081: F, t36085: F, t36087: F, t36089: F, t36096: F, t36125: F, t36131: F, t36133: F, t36151: F, t36156: F, t36162: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37830 = 0.68598428988911579156e-2 * t36017;
    let t37833 = 0.62896184579208304138e-3 * t36030;
    let t37836 = 7.0 / 12.0 * t36039;
    let t37837 = 7.0 / 36.0 * t36041;
    let t37848 = 11.0 / 144.0 * t36065;
    let t37857 = 0.12579236915841660828e-2 * t36081;
    let t37859 = 0.21437009059034868486e-2 * t36085;
    let t37860 = 0.85748036236139473944e-3 * t36087;
    let t37861 = 0.42874018118069736972e-3 * t36089;
    let t37864 = 0.62896184579208304138e-3 * t36096;
    let t37872 = 0.32012600194825403606e-1 * t36125;
    let t37875 = 0.85748036236139473944e-3 * t36131;
    let t37876 = 0.85748036236139473944e-3 * t36133;
    let t37888 = 7.0 / 72.0 * t36151;
    let t37892 = 0.12579236915841660828e-2 * t36156;
    let t37894 = 0.85748036236139473944e-3 * t36162;
    (t37830, t37833, t37836, t37837, t37848, t37857, t37859, t37860, t37861, t37864, t37872, t37875, t37876, t37888, t37892, t37894)
}
