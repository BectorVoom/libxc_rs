//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 794/1208 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk794<F: Float>(t158: F, t8839: F, t8840: F, t8847: F, t8856: F, t1029: F, t133: F, t1773: F, t3401: F, t568: F, t2575: F, t2632: F, t3396: F, t614: F, t596: F, t8817: F) -> (F, F, F, F, F, F, F) {
    let t8859 = (t8839 + t8840 + t8847 + t8856) * t158;
    let t8865 = t1029 * t133;
    let t8872 = t1773 * t3401;
    let t8873 = t8872 * t568;
    let t8876 = t2632 * t2575;
    let t8881 = t614 * t3396;
    let t8882 = t8881 * t568;
    let t8885 = t596 * t8817;
    (t8859, t8865, t8872, t8873, t8876, t8882, t8885)
}
