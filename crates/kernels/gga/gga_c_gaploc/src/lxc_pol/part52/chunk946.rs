//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 946/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk946<F: Float>(t2478: F, t3695: F, t6576: F, t2482: F, t9263: F, t107: F, t47008: F, t544: F, t3689: F, t4130: F, t9272: F, t12079: F, t2389: F) -> (F, F, F, F, F) {
    let t47829 = t6576 * t3695 * t2478;
    let t47832 = t9263 * t3695 * t2482;
    let t47838 = t544 * t47008 * t107;
    let t47848 = t4130 * t3689;
    let t47850 = t9272 * t47848 * t2482;
    let t47866 = t12079 * t2389;
    (t47829, t47832, t47838, t47850, t47866)
}
