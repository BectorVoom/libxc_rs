//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 903/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk903<F: Float>(t30402: F, t30407: F, t30409: F, t372: F, t141: F, t7335: F, t301: F, t7325: F, t1016: F, t1072: F, t30418: F, t2019: F, t2028: F) -> (F, F, F, F, F, F) {
    let t31095 = t30407 * t30402 * t30409 * t372;
    let t31097 = t7335 * t141;
    let t31100 = t30407 * t31097 * t7325 * t301;
    let t31102 = t1016 * t1072;
    let t31105 = t30407 * t30418 * t31102 * t372;
    let t31110 = t2019 * t2028;
    (t31095, t31097, t31100, t31102, t31105, t31110)
}
