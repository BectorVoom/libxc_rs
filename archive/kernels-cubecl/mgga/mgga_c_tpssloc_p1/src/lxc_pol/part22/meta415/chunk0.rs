//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1718/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1718<F: Float>(t1100: F, t18730: F, t1107: F, t11243: F, t5992: F, t1102: F, t4756: F, t4764: F, t3287: F, t5999: F, t11265: F, t4748: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18731 = t1100 * t18730;
    let t18742 = t1107 * t18730;
    let t18746 = t11243 * t5992;
    let t18747 = t18746 * t1102;
    let t18749 = t4764 * t4756;
    let t18751 = t3287 * t5999;
    let t18752 = t18751 * t1102;
    let t18754 = t11265 * t5992;
    let t18755 = t18754 * t1102;
    let t18757 = t4748 * t4756;
    (t18731, t18742, t18746, t18747, t18749, t18752, t18754, t18755, t18757)
}
