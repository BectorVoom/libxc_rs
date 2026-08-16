//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2539/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2539<F: Float>(t10771: F, t1568: F, t10756: F, t1580: F, t2930: F, t2885: F, t4408: F, t10813: F, t4433: F, t13716: F, t2932: F, t10632: F, t4471: F) -> (F, F, F, F, F, F, F) {
    let t48776 = t10771 * t1568;
    let t48779 = t10756 * t1580;
    let t48783 = t2930 * t1580;
    let t48789 = t4408 * t2885;
    let t48854 = t4433 * t10813;
    let t48883 = t13716 * t2932;
    let t48890 = t4471 * t10632;
    (t48776, t48779, t48783, t48789, t48854, t48883, t48890)
}
