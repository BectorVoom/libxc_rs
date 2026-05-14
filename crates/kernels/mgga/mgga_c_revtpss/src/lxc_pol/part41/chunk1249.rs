//! MGGA_C_REVTPSS lxc pol — lxc_pol part 41 (v4rho3tau_4) CSE chunk 1249/1356 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part41_v4rho3tau_4_chunk1249<F: Float>(t5378: F, t5391: F, t17459: F, t6688: F, t3720: F, t5405: F, t6421: F, t12787: F, t17394: F, t4890: F, t3767: F, t3782: F, t3628: F, t4186: F, t5351: F, t3626: F) -> (F, F, F, F, F, F) {
    let t21001 = t5391 * t5378;
    let t21003 = t6688 * t17459;
    let t21004 = t3720 * t21003;
    let t21007 = t6421 * t5405;
    let t21008 = t12787 * t21007;
    let t21013 = t17394 * t4890;
    let t21014 = t3767 * t21013;
    let t21017 = t3782 * t21013;
    let t21020 = t3628 * t4186;
    let t21021 = t5351 * t21020;
    let t21022 = t3626 * t21021;
    (t21001, t21004, t21008, t21014, t21017, t21022)
}
