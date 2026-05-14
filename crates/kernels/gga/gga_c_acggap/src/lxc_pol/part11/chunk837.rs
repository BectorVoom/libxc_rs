//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 837/1066 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk837<F: Float>(t301: F, t30407: F, t31097: F, t7325: F, t1016: F, t1072: F, t30418: F, t372: F, t3201: F, t7486: F, t2095: F, t2019: F, t2028: F, t1152: F, t1: F, t2065: F, t7335: F) -> (F, F, F, F, F, F, F) {
    let t31100 = t30407 * t31097 * t7325 * t301;
    let t31102 = t1016 * t1072;
    let t31105 = t30407 * t30418 * t31102 * t372;
    let t31107 = t3201 * t7486;
    let t31108 = t2095 * t31107;
    let t31110 = t2019 * t2028;
    let t31111 = t31110 * t1152;
    let t31114 = t2065 * t7335 * t1;
    (t31100, t31102, t31105, t31107, t31108, t31111, t31114)
}
