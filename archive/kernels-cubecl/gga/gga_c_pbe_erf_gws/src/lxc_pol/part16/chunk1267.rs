//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1267/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1267<F: Float>(t14064: F, t3120: F, t14031: F, t9372: F, t51421: F, t9512: F, t14007: F, t9570: F, t4023: F, t9179: F, t6645: F, t8991: F) -> (F, F, F, F, F, F) {
    let t54029 = t3120 * t14064;
    let t54031 = t14031 * t9372;
    let t54033 = t51421 * t9512;
    let t54035 = t14007 * t9570;
    let t54039 = t9179 * t4023;
    let t54043 = t6645 * t8991;
    (t54029, t54031, t54033, t54035, t54039, t54043)
}
