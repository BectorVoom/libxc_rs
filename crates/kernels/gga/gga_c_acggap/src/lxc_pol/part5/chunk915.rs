//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 915/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk915<F: Float>(t1181: F, t3391: F, t3529: F, t4643: F, t3759: F, t10098: F, t3402: F, t4469: F, t13860: F, t4925: F, t1413: F, t3476: F, t384: F, t398: F, t429: F, t5087: F) -> (F, F, F, F, F, F, F) {
    let t16893 = t3391 * t1181 * t4643 * t3529;
    let t16897 = t3391 * t1181 * t4643 * t3759;
    let t16899 = t10098 * t3402;
    let t16900 = t16899 * t4469;
    let t16902 = t13860 * t4925;
    let t16911 = t3476 * t1413;
    let t16916 = t384 * t398 * t429 * t5087;
    (t16893, t16897, t16899, t16900, t16902, t16911, t16916)
}
