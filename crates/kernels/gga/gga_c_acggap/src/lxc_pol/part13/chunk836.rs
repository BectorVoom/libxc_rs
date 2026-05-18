//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 836/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk836<F: Float>(t435: F, t965: F, t1159: F, t848: F, t1111: F, t301: F, t182: F, t862: F, t1083: F, t171: F) -> (F, F, F, F, F) {
    let t12610 = t965 * t435;
    let t12726 = t848 * t1159;
    let t12816 = t1111 * t301;
    let t12935 = t862 * t182;
    let t13287 = t171 * t1083;
    (t12610, t12726, t12816, t12935, t13287)
}
