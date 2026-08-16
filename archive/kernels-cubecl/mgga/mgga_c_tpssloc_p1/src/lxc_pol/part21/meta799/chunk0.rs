//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2779/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2779<F: Float>(t16673: F, t2696: F, t849: F, t13360: F, t4261: F, t5584: F, t9975: F, t5619: F, t9674: F, t4250: F, t46657: F, t16907: F, t9638: F) -> (F, F, F, F, F, F) {
    let t58844 = t16673 * t2696;
    let t58845 = t58844 * t849;
    let t58847 = t13360 * t4261;
    let t58853 = t5584 * t9975;
    let t58859 = t9674 * t5619;
    let t58873 = t46657 * t4250;
    let t58885 = t9638 * t16907;
    (t58845, t58847, t58853, t58859, t58873, t58885)
}
