//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1132/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1132<F: Float>(t3295: F, t4039: F, t4022: F, t863: F, t6523: F, t8867: F, t1150: F, t14028: F, t14046: F, t4171: F, t3268: F, t4049: F) -> (F, F, F, F, F, F, F) {
    let t14544 = t4039 * t3295;
    let t14547 = t863 * t4022;
    let t14548 = t6523 * t8867;
    let t14549 = t14547 * t14548;
    let t14551 = t14028 * t1150;
    let t14554 = t14046 * t4171;
    let t14556 = t4049 * t3268;
    (t14544, t14547, t14548, t14549, t14551, t14554, t14556)
}
