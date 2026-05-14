//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 926/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk926<F: Float>(t309: F, t406: F, t944: F, t33673: F, t7963: F, t524: F, t7932: F, t4210: F, t7942: F, t7965: F, t2131: F, t2132: F, t8993: F, t2138: F, t2147: F, t322: F, t8392: F) -> (F, F, F, F, F, F) {
    let t36429 = t944 * t309 * t406;
    let t36432 = 0.34694512752820797848e1 * t7963 * t33673 * t36429;
    let t36433 = t7932 * t524;
    let t36436 = 0.17347256376410398924e1 * t7942 * t36433 * t4210;
    let t36439 = 0.17347256376410398924e1 * t7963 * t36433 * t7965;
    let t36447 = 0.17347256376410398924e1 * t2131 * t2132 * t8993 * t309;
    let t36452 = 0.34694512752820797848e1 * t2138 * t2147 * t8392 * t322;
    (t36432, t36433, t36436, t36439, t36447, t36452)
}
