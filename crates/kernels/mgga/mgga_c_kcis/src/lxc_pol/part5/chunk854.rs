//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 854/1260 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk854<F: Float>(t8778: F, t2740: F, t882: F, t209: F, t207: F, t69: F, t2739: F, t6: F, t2746: F, t2709: F, t8630: F, t864: F, t8640: F, t8646: F, t8649: F, t8653: F, t8660: F, t8666: F, t8669: F, t867: F, t8674: F, t8678: F, t8717: F) -> (F, F, F, F) {
    let t8779 = 1.0 / t8778;
    let t8780 = t2740 * t882;
    let t8782 = t209 * t8779 * t8780;
    let t8785 = t207 * t69;
    let t8786 = t6 * t2739;
    let t8788 = t8786 * t882 * t2746;
    let t8797 = t8630 - 0.1025389702100779493e4 * t867 * t8660 + t8646 - t8649 - t8653 - 0.56969282336565386482e-3 * t864 * t8717 + 0.48159446095139119799e0 * t2709 * t8640 + t8666 - t8669 + t8674 + t8678;
    (t8782, t8785, t8788, t8797)
}
