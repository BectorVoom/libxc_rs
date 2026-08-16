//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 946/1332 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk946<F: Float>(t229: F, t4059: F, t1378: F, t40: F, t803: F, t2824: F, t483: F, t1388: F, t709: F, t301: F, t96: F, t4068: F) -> (F, F, F, F, F, F) {
    let t14919 = t229 * t4059;
    let t14930 = t40 * t1378 * t803;
    let t14935 = t40 * t483 * t2824;
    let t14941 = t709 * t1388;
    let t14947 = t96 * t301;
    let t14957 = t229 * t4068;
    (t14919, t14930, t14935, t14941, t14947, t14957)
}
