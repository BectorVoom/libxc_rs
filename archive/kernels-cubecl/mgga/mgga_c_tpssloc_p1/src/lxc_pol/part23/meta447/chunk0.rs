//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1292/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1292<F: Float>(t12250: F, t6414: F, t1338: F, t20601: F, t12461: F, t20684: F, t571: F, t6330: F, t20193: F, t604: F, t1409: F, t1426: F, t67: F) -> (F, F, F, F, F, F) {
    let t75008 = t12250 * t6414;
    let t75124 = t1338 * t20601;
    let t75240 = t20684 * t12461;
    let t75256 = t6330 * t571;
    let t75284 = t20193 * t604;
    let t75361 = t1409 * t1426 * t67;
    (t75008, t75124, t75240, t75256, t75284, t75361)
}
