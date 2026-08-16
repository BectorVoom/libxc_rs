//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1223/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1223<F: Float>(t1210: F, t3500: F, t65539: F, t15734: F, t5005: F, t11818: F, t248: F, t3506: F, t6225: F, t3540: F, t6170: F, t6158: F) -> (F, F, F, F, F) {
    let t65545 = t3500 * t1210 * t65539;
    let t65552 = t5005 * t15734;
    let t65558 = t3506 * t248 * t11818 * t6225;
    let t65581 = t6170 * t3540;
    let t65600 = t6158 * t3540;
    (t65545, t65552, t65558, t65581, t65600)
}
