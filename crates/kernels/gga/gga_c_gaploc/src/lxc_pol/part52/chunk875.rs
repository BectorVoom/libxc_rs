//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 875/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk875<F: Float>(t502: F, t50493: F, t50533: F, t50556: F, t50583: F, t50594: F, t50606: F, t50611: F, t50647: F, t50661: F, t50675: F, t50688: F, t50717: F, t50750: F, t50757: F, t50763: F, t50776: F) -> (F,) {
    let t50781 = t502 * (t50493 + t50533 + t50556 + t50583 + t50594 + t50606 + t50611 + t50647 + t50661 + t50675 + t50688 + t50717 + t50750 + t50757 + t50763 + t50776);
    (t50781,)
}
