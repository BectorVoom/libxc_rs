//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 641/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk641<F: Float>(t2062: F, t7440: F, t1017: F, t7351: F, t142: F, t2060: F, t2015: F, t2029: F) -> (F, F, F, F, F, F) {
    let t7441 = t7440 * t2062;
    let t7442 = 0.5603125e-1 * t7441;
    let t7443 = t7351 * t1017;
    let t7444 = t142 * t7443;
    let t7445 = t2060 * t7444;
    let t7447 = t2015 * t2029;
    (t7441, t7442, t7443, t7444, t7445, t7447)
}
