//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 493/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk493<F: Float>(t9102: F, t9105: F, t4074: F, t4077: F, t4082: F, t4085: F, t2282: F, t3101: F, t3106: F, t467: F, t2312: F, t3122: F) -> (F, F, F, F, F, F) {
    let t9106 = t9102 * t9105;
    let t9108 = t9106 * t4074 * t4077;
    let t9111 = t4082 * t9106 * t4085;
    let t9113 = t3101 * t2282;
    let t9115 = t3106 * t467;
    let t9147 = F::cast_from(0.23712505529730124666e-2_f64) * t2312 * t3122;
    (t9106, t9108, t9111, t9113, t9115, t9147)
}
