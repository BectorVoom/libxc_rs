//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 767/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk767<F: Float>(t107: F, t36610: F, t787: F, t11844: F, t1980: F, t11848: F, t35445: F, t739: F, t35439: F, t11613: F, t769: F, t11822: F) -> (F, F, F, F, F, F, F) {
    let t36612 = t787 * t36610 * t107;
    let t36632 = t1980 * t11844;
    let t36635 = t1980 * t11848;
    let t36654 = t739 * t35445;
    let t36700 = t787 * t35439 * t107;
    let t36738 = t769 * t11613;
    let t36762 = t1980 * t11822;
    (t36612, t36632, t36635, t36654, t36700, t36738, t36762)
}
