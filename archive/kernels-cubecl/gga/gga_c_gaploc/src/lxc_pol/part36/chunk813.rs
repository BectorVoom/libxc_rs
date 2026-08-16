//! GGA_C_GAPLOC lxc pol — lxc_pol part 36 (v4rhosigma3_1) CSE chunk 813/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part36_v4rhosigma3_1_chunk813<F: Float>(t22629: F, t825: F, t9438: F, t900: F, t9624: F, t10023: F, t10032: F, t2021: F, t7372: F, t2673: F, t40848: F, t41416: F, t969: F) -> (F, F, F, F, F, F) {
    let t41477 = t825 * t9438 * t22629;
    let t41511 = t900 * t9624;
    let t41512 = t10023 * t41511;
    let t41515 = t2021 * t10032 * t7372;
    let t41518 = t2673 * t900 * t40848;
    let t41528 = t825 * t969 * t41416;
    (t41477, t41511, t41512, t41515, t41518, t41528)
}
