//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 844/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk844<F: Float>(t19561: F, t2105: F, t825: F, t2365: F, t4395: F, t337: F, t6658: F, t2306: F, t4422: F, t56: F, t2118: F, t816: F, t6158: F, t328: F, t6045: F, t824: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19562 = t19561 * t2105;
    let t19563 = t19562 * t825;
    let t19637 = t4395 * t2365;
    let t19693 = t6658 * t337;
    let t19733 = t2306 * t4422;
    let t19775 = t2365 * t56;
    let t19776 = t2118 * t19775;
    let t19803 = t19561 * t816;
    let t19804 = t19803 * t825;
    let t19810 = t6158 * t2365;
    let t19817 = t2118 * t4422;
    let t19839 = t824 * t328 * t6045;
    (t19562, t19563, t19637, t19693, t19733, t19775, t19776, t19803, t19804, t19810, t19817, t19839)
}
