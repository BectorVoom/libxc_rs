//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 973/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk973<F: Float>(t142: F, t5170: F, t8888: F, t5164: F, t8806: F, t2060: F, t507: F, t7443: F, t7450: F, t7451: F, t7447: F, t8813: F, t8817: F, t7440: F, t8820: F, t2274: F, t30307: F) -> (F, F, F, F, F, F, F, F) {
    let t35059 = t8888 * t142 * t5170;
    let t35062 = t8806 * t142 * t5164;
    let t35065 = t2060 * t507 * t7443;
    let t35068 = t7450 * t507 * t7451;
    let t35070 = t7447 * t8813;
    let t35071 = 0.84046875e-1 * t35070;
    let t35072 = t7447 * t8817;
    let t35073 = 0.84046875e-1 * t35072;
    let t35074 = t7440 * t8820;
    let t35075 = 0.5603125e-1 * t35074;
    let t35076 = t30307 * t2274;
    (t35059, t35062, t35065, t35068, t35071, t35073, t35075, t35076)
}
