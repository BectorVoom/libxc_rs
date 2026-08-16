//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1189/1527 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1189<F: Float>(t11727: F, t52835: F, t11832: F, t1706: F, t11887: F, t52834: F, t11913: F, t11880: F, t15908: F, t9467: F, t9882: F, t5154: F, t9919: F) -> (F, F, F, F, F, F, F, F) {
    let t53472 = t52835 * t11727;
    let t53490 = t1706 * t11832;
    let t53565 = t52834 * t11887;
    let t53592 = t52834 * t11913;
    let t53613 = t52834 * t11880;
    let t53777 = t15908 * t9467;
    let t53779 = t15908 * t9882;
    let t53798 = t5154 * t9919;
    (t53472, t53490, t53565, t53592, t53613, t53777, t53779, t53798)
}
