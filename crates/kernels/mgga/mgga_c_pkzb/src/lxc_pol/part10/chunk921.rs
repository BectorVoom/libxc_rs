//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 921/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk921<F: Float>(t1029: F, t1031: F, t160: F, t162: F, t1742: F, t1747: F, t1750: F, t2625: F, t2631: F, t2633: F, t2636: F, t594: F, t597: F, t7055: F, t7065: F, t7071: F, t7075: F, t7078: F, t7081: F) -> (F,) {
    let t7084 = -12.0 * t1029 * t1747 + 3.0 * t1029 * t1750 + 3.0 * t1031 * t1742 + 3.0 * t160 * t7081 - t162 * t7055 + 6.0 * t2625 * t597 + 60.0 * t2631 * t7071 - 24.0 * t2631 * t7075 - 12.0 * t2631 * t7078 - 24.0 * t2633 * t7065 + 6.0 * t2636 * t594;
    (t7084,)
}
