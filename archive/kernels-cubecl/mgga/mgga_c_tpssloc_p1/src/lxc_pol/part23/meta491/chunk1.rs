//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1503/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1503<F: Float>(t39312: F, t39316: F, t39320: F, t39324: F, t39327: F, t39338: F, t39346: F, t39349: F, t39356: F, t79856: F, t79857: F, t79858: F, t79890: F) -> F {
    let t80102 = t39312 + t39316 + t39320 - t39324 - t79856 - t79857 + t79858 + t39327 - t39338 + t39346 + t39349 + t79890 + t39356;
    t80102
}
