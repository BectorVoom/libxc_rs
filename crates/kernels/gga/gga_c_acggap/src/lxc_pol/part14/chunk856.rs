//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 856/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk856<F: Float>(t8396: F, t862: F, t7898: F, t315: F, t323: F, t8993: F, t7908: F, t8998: F, t33489: F, t7963: F, t7965: F, t4210: F, t7942: F, t5386: F, t610: F, t2132: F, t322: F, t7896: F, t8422: F) -> (F, F, F, F, F, F, F) {
    let t33574 = t862 * t8396;
    let t33575 = t33574 * t7898;
    let t33586 = 0.13170898365871023197e1 * t315 * t8993 * t323;
    let t33606 = 0.34694512752820797848e1 * t8998 * t7908;
    let t33621 = 0.17347256376410398924e1 * t7963 * t33489 * t7965;
    let t33624 = 0.17347256376410398924e1 * t7942 * t33489 * t4210;
    let t33627 = 0.26341796731742046394e1 * t315 * t610 * t5386;
    let t33635 = t7896 * t2132 * t8422 * t322;
    (t33575, t33586, t33606, t33621, t33624, t33627, t33635)
}
