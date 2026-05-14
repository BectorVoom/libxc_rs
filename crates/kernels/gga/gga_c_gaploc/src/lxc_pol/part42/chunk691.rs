//! GGA_C_GAPLOC lxc pol — lxc_pol part 42 (v4rhosigma3_7) CSE chunk 691/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part42_v4rhosigma3_7_chunk691<F: Float>(t40820: F, t900: F, t22624: F, t7427: F, t9438: F, t22634: F, t2684: F, t22629: F, t825: F, t9624: F, t12960: F, t1537: F, t34890: F, t6583: F, t9537: F, t10473: F, t2482: F, t9263: F) -> (F, F, F, F, F, F, F, F) {
    let t41339 = t900 * t40820;
    let t41408 = t7427 * t9438 * t22624;
    let t41448 = t2684 * t9438 * t22634;
    let t41477 = t825 * t9438 * t22629;
    let t41511 = t900 * t9624;
    let t41594 = t1537 * t12960;
    let t41606 = t6583 * t34890 * t9537;
    let t41609 = t9263 * t10473 * t2482;
    (t41339, t41408, t41448, t41477, t41511, t41594, t41606, t41609)
}
