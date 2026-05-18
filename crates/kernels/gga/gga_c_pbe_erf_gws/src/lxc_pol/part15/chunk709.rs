//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 15 (v4rho3sigma_3) CSE chunk 709/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part15_v4rho3sigma_3_chunk709<F: Float>(t3959: F, t4130: F, t1146: F, t1173: F, t2409: F, t3045: F, t3965: F, t1149: F, t3975: F, t3972: F, t1118: F, t1178: F) -> (F, F, F, F, F, F, F) {
    let t4131 = t3959 * t4130;
    let t4133 = t1173 * t1146;
    let t4135 = t2409 * t3045;
    let t4136 = t3965 * t4135;
    let t4138 = t3975 * t1149;
    let t4139 = t3972 * t4138;
    let t4141 = t1178 * t1118;
    (t4131, t4133, t4135, t4136, t4138, t4139, t4141)
}
