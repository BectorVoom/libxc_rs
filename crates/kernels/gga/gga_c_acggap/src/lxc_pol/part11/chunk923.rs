//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 923/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk923<F: Float>(t2019: F, t2028: F, t1152: F, t1: F, t2065: F, t7335: F, t1160: F) -> (F, F, F) {
    let t31110 = t2019 * t2028;
    let t31111 = t31110 * t1152;
    let t31114 = t2065 * t7335 * t1;
    let t31115 = t1160 * t31114;
    (t31111, t31114, t31115)
}
